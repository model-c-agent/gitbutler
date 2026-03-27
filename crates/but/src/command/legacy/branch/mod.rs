use anyhow::bail;
use but_api::json::HexHash;
use but_core::ref_metadata::StackId;
use colored::Colorize;

use crate::{
    CliId, IdMap,
    utils::{Confirm, ConfirmDefault, OutputChannel, shorten_object_id},
};

pub mod apply;
mod json;
mod list;
mod show;

pub fn delete(
    ctx: &mut but_ctx::Context,
    out: &mut OutputChannel,
    branch_name: Option<String>,
    pattern: Option<String>,
    force: bool,
) -> Result<(), anyhow::Error> {
    let stacks = but_api::legacy::workspace::stacks(
        ctx,
        Some(but_workspace::legacy::StacksFilter::InWorkspace),
    )?;

    if let Some(pattern) = pattern {
        // Pattern-based bulk deletion
        let regex = regex::Regex::new(&pattern)
            .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {e}"))?;

        let mut matches: Vec<(StackId, String)> = Vec::new();
        for stack_entry in &stacks {
            if let Some(sid) = stack_entry.id {
                for head in &stack_entry.heads {
                    if regex.is_match(&head.name.to_string()) {
                        matches.push((sid, head.name.to_string()));
                    }
                }
            }
        }

        if matches.is_empty() {
            if let Some(out) = out.for_human() {
                writeln!(out, "No branches matching pattern '{pattern}'")?;
            }
            return Ok(());
        }

        if let Some(out) = out.for_human() {
            writeln!(out, "Branches matching '{pattern}':")?;
            for (_, name) in &matches {
                writeln!(out, "  {name}")?;
            }
        }

        if !force
            && let Some(mut inout) = out.prepare_for_terminal_input()
            && inout.confirm(
                format!("Delete {} branch(es)?", matches.len()),
                ConfirmDefault::No,
            )? == Confirm::No
        {
            bail!("Aborted.");
        }

        let mut deleted = 0;
        for (sid, name) in &matches {
            if let Err(e) = but_api::legacy::stack::remove_branch(ctx, *sid, name.clone()) {
                if let Some(out) = out.for_human() {
                    writeln!(out, "Failed to delete {name}: {e:#}")?;
                }
            } else {
                deleted += 1;
            }
        }

        if let Some(out) = out.for_human() {
            writeln!(out, "Deleted {deleted}/{} branch(es).", matches.len())?;
        }
        Ok(())
    } else if let Some(branch_name) = branch_name {
        // Single branch deletion
        for stack_entry in &stacks {
            if stack_entry.heads.iter().all(|b| b.name != *branch_name) {
                continue;
            }

            if let Some(sid) = stack_entry.id {
                return confirm_branch_deletion(ctx, sid, &branch_name, force, out);
            }
        }

        if let Some(out) = out.for_human() {
            writeln!(out, "Branch '{branch_name}' not found in any stack")?;
        }
        Ok(())
    } else {
        bail!("Either a branch name or --pattern is required");
    }
}

pub fn rename(
    ctx: &mut but_ctx::Context,
    out: &mut OutputChannel,
    old_name: String,
    new_name: String,
) -> Result<(), anyhow::Error> {
    let stacks = but_api::legacy::workspace::stacks(
        ctx,
        Some(but_workspace::legacy::StacksFilter::InWorkspace),
    )?;

    for stack_entry in &stacks {
        if stack_entry.heads.iter().all(|b| b.name != *old_name) {
            continue;
        }

        if let Some(sid) = stack_entry.id {
            but_api::legacy::stack::update_branch_name(
                ctx,
                sid,
                old_name.clone(),
                new_name.clone(),
            )?;

            if let Some(out) = out.for_human() {
                writeln!(
                    out,
                    "{} {} → {}",
                    "✓ Renamed".green(),
                    old_name.dimmed(),
                    new_name.yellow()
                )?;
            } else if let Some(out) = out.for_shell() {
                writeln!(out, "{new_name}")?;
            } else if let Some(out) = out.for_json() {
                out.write_value(json::BranchRenameOutput {
                    old_name: old_name.clone(),
                    new_name: new_name.clone(),
                })?;
            }
            return Ok(());
        }
    }

    if let Some(out) = out.for_human() {
        writeln!(out, "Branch '{old_name}' not found in any stack")?;
    }
    Ok(())
}

pub fn new(
    ctx: &mut but_ctx::Context,
    out: &mut OutputChannel,
    branch_name: Option<String>,
    anchor: Option<String>,
) -> Result<(), anyhow::Error> {
    let id_map = IdMap::new_from_context(ctx, None)?;
    // Get branch name or use canned name
    let branch_name = branch_name
        .map(Ok)
        .unwrap_or_else(|| but_api::legacy::workspace::canned_branch_name(ctx))?;

    // Store anchor string for JSON output
    let anchor_for_json = anchor.clone();

    let anchor = if let Some(anchor_str) = anchor {
        // Use the new create_reference API when anchor is provided

        // Resolve the anchor string to a CliId
        let anchor_ids = id_map.parse_using_context(&anchor_str, ctx)?;
        if anchor_ids.is_empty() {
            return Err(anyhow::anyhow!("Could not find anchor: {anchor_str}"));
        }
        if anchor_ids.len() > 1 {
            return Err(anyhow::anyhow!(
                "Ambiguous anchor '{anchor_str}', matches multiple items"
            ));
        }
        let anchor_id = &anchor_ids[0];

        // Create the anchor for create_reference
        // as dependent branch
        match anchor_id {
            CliId::Commit { commit_id: oid, .. } => {
                Some(but_api::legacy::stack::create_reference::Anchor::AtCommit {
                    commit_id: HexHash(*oid),
                    position: but_workspace::branch::create_reference::Position::Above,
                })
            }
            CliId::Branch { name, .. } => Some(
                but_api::legacy::stack::create_reference::Anchor::AtReference {
                    short_name: name.clone(),
                    position: but_workspace::branch::create_reference::Position::Above,
                },
            ),
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid anchor type: {}, expected commit or branch",
                    anchor_id.kind_for_humans()
                ));
            }
        }
    } else {
        // Create an independent branch
        None
    };

    let anchor_display = {
        let repo = ctx.repo.get()?;
        anchor.as_ref().map(|anchor_ref| match anchor_ref {
            but_api::legacy::stack::create_reference::Anchor::AtReference {
                short_name, ..
            } => short_name.clone(),
            but_api::legacy::stack::create_reference::Anchor::AtCommit { commit_id, .. } => {
                shorten_object_id(&repo, commit_id.0)
            }
        })
    };

    but_api::legacy::stack::create_reference(
        ctx,
        but_api::legacy::stack::create_reference::Request {
            new_name: branch_name.clone(),
            anchor,
        },
    )?;

    if let Some(out) = out.for_human() {
        if let Some(anchor_name) = anchor_display {
            writeln!(
                out,
                "{} {} stacked on {}",
                "✓ Created branch".green(),
                branch_name.yellow(),
                anchor_name.dimmed()
            )?;
        } else {
            writeln!(
                out,
                "{} {}",
                "✓ Created branch".green(),
                branch_name.yellow()
            )?;
        }
    } else if let Some(out) = out.for_shell() {
        writeln!(out, "{branch_name}")?;
    } else if let Some(out) = out.for_json() {
        let value = json::BranchNewOutput {
            branch: branch_name.clone(),
            anchor: anchor_for_json,
        };
        out.write_value(value)?;
    }
    Ok(())
}

pub fn show_branches(
    ctx: &mut but_ctx::Context,
    out: &mut OutputChannel,
    branch_id: String,
    review: bool,
    files: bool,
    ai: bool,
    check: bool,
) -> Result<(), anyhow::Error> {
    show::show(ctx, &branch_id, out, review, files, ai, check)?;
    Ok(())
}

#[expect(clippy::too_many_arguments)]
pub fn list_branches(
    ctx: &mut but_ctx::Context,
    out: &mut OutputChannel,
    filter: Option<String>,
    pattern: Option<String>,
    local: bool,
    remote: bool,
    all: bool,
    no_ahead: bool,
    review: bool,
    no_check: bool,
    empty: bool,
) -> Result<(), anyhow::Error> {
    let ahead = !no_ahead;
    // Invert the flag
    let check = !no_check;
    // Invert the flag
    list::list(
        ctx, local, remote, all, ahead, review, filter, pattern, out, check, empty,
    )?;
    Ok(())
}

pub fn handle_no_subcommand(
    ctx: &mut but_ctx::Context,
    out: &mut OutputChannel,
) -> Result<(), anyhow::Error> {
    list_branches(
        ctx, out, None, None, false, false, false, false, false, false, false,
    )
}

fn confirm_branch_deletion(
    ctx: &mut but_ctx::Context,
    sid: StackId,
    branch_name: &str,
    force: bool,
    out: &mut OutputChannel,
) -> Result<(), anyhow::Error> {
    if !force
        && let Some(mut inout) = out.prepare_for_terminal_input()
        && inout.confirm(
            format!("Are you sure you want to delete branch '{branch_name}'?"),
            ConfirmDefault::No,
        )? == Confirm::No
    {
        bail!("Aborted branch deletion.");
    }

    but_api::legacy::stack::remove_branch(ctx, sid, branch_name.to_owned())?;

    if let Some(out) = out.for_human() {
        writeln!(out, "Deleted branch {branch_name}")?;
    }
    Ok(())
}
