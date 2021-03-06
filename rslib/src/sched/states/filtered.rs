// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use crate::revlog::RevlogReviewKind;

use super::{IntervalKind, NextCardStates, PreviewState, ReschedulingFilterState, StateContext};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilteredState {
    Preview(PreviewState),
    Rescheduling(ReschedulingFilterState),
}

impl FilteredState {
    pub(crate) fn interval_kind(self) -> IntervalKind {
        match self {
            FilteredState::Preview(state) => state.interval_kind(),
            FilteredState::Rescheduling(state) => state.interval_kind(),
        }
    }

    pub(crate) fn revlog_kind(self) -> Option<RevlogReviewKind> {
        match self {
            FilteredState::Preview(_state) => None,
            FilteredState::Rescheduling(state) => Some(state.revlog_kind()),
        }
    }

    pub(crate) fn next_states(self, ctx: &StateContext) -> NextCardStates {
        match self {
            FilteredState::Preview(state) => state.next_states(ctx),
            FilteredState::Rescheduling(state) => state.next_states(ctx),
        }
    }
}
