// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use crate::revlog::RevlogReviewKind;

use super::{
    interval_kind::IntervalKind, LearnState, NewState, NextCardStates, RelearnState, ReviewState,
    StateContext,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalState {
    New(NewState),
    Learning(LearnState),
    Review(ReviewState),
    Relearning(RelearnState),
}

impl NormalState {
    pub(crate) fn interval_kind(self) -> IntervalKind {
        match self {
            NormalState::New(state) => state.interval_kind(),
            NormalState::Learning(state) => state.interval_kind(),
            NormalState::Review(state) => state.interval_kind(),
            NormalState::Relearning(state) => state.interval_kind(),
        }
    }

    pub(crate) fn revlog_kind(self) -> RevlogReviewKind {
        match self {
            NormalState::New(state) => state.revlog_kind(),
            NormalState::Learning(state) => state.revlog_kind(),
            NormalState::Review(state) => state.revlog_kind(),
            NormalState::Relearning(state) => state.revlog_kind(),
        }
    }

    pub(crate) fn next_states(self, ctx: &StateContext) -> NextCardStates {
        match self {
            NormalState::New(_) => {
                // New state acts like answering a failed learning card
                let next_states = LearnState {
                    remaining_steps: ctx.steps.remaining_for_failed(),
                    scheduled_secs: 0,
                }
                .next_states(ctx);
                // .. but with current as New, not Learning
                NextCardStates {
                    current: self.into(),
                    ..next_states
                }
            }
            NormalState::Learning(state) => state.next_states(ctx),
            NormalState::Review(state) => state.next_states(ctx),
            NormalState::Relearning(state) => state.next_states(ctx),
        }
    }
}

impl From<NewState> for NormalState {
    fn from(state: NewState) -> Self {
        NormalState::New(state)
    }
}

impl From<ReviewState> for NormalState {
    fn from(state: ReviewState) -> Self {
        NormalState::Review(state)
    }
}

impl From<LearnState> for NormalState {
    fn from(state: LearnState) -> Self {
        NormalState::Learning(state)
    }
}

impl From<RelearnState> for NormalState {
    fn from(state: RelearnState) -> Self {
        NormalState::Relearning(state)
    }
}
