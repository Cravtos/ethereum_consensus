use crate::phase0::beacon_state::BeaconState;
use crate::phase0::operations::PendingAttestation;
use crate::phase0::state_transition::{
    get_block_root, get_current_epoch, get_previous_epoch, Context, Error,
};
use crate::primitives::Epoch;
use ssz_rs::prelude::*;

pub fn process_epoch<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    _state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    _context: &Context,
) -> Result<(), Error> {
    Ok(())
}

pub fn get_matching_source_attestations<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    context: &Context,
) -> Result<
    &'a List<PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>, PENDING_ATTESTATIONS_BOUND>,
    Error,
> {
    let previous_epoch = get_previous_epoch(state, context);
    let current_epoch = get_current_epoch(state, context);

    if previous_epoch != epoch && current_epoch != epoch {
        return Err(Error::InvalidEpoch {
            requested: epoch,
            previous: previous_epoch,
            current: current_epoch,
        });
    }

    if epoch == current_epoch {
        Ok(&state.current_epoch_attestations)
    } else {
        Ok(&state.previous_epoch_attestations)
    }
}

pub fn get_matching_target_attestations<
    'a,
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const PENDING_ATTESTATIONS_BOUND: usize,
>(
    state: &'a BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        PENDING_ATTESTATIONS_BOUND,
    >,
    epoch: Epoch,
    context: &Context,
) -> Result<impl Iterator<Item = &'a PendingAttestation<MAX_VALIDATORS_PER_COMMITTEE>>, Error> {
    let source_attestations = get_matching_source_attestations(state, epoch, context)?;
    let block_root = get_block_root(state, epoch, context)?;
    Ok(source_attestations
        .into_iter()
        .filter(|a| a.data.target.root.as_ref() == block_root.as_ref()))
}
