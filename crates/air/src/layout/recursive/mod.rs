pub mod autogenerated;
pub mod global_values;

use crate::{
    diluted::get_diluted_product,
    periodic_columns::{eval_pedersen_x, eval_pedersen_y},
    public_memory::{PublicInput, INITIAL_PC, MAX_ADDRESS, MAX_LOG_N_STEPS, MAX_RANGE_CHECK},
};
use bail_out::ensure;
use cairovm_verifier_commitment::table::{commit::table_commit, decommit::table_decommit};
use global_values::{EcPoint, GlobalValues, InteractionElements};
use starknet_core::types::NonZeroFelt;
use starknet_crypto::{poseidon_hash_many, Felt};

use super::{CompositionPolyEvalError, LayoutTrait, PublicInputError};

// Recursive layout consts
pub const BITWISE_RATIO: u32 = 8;
pub const BITWISE_ROW_RATIO: u32 = 128;
pub const BITWISE_TOTAL_N_BITS: u32 = 251;
pub const CONSTRAINT_DEGREE: u32 = 2;
pub const CPU_COMPONENT_HEIGHT: u32 = 16;
pub const CPU_COMPONENT_STEP: u32 = 1;
pub const DILUTED_N_BITS: u32 = 16;
pub const DILUTED_SPACING: u32 = 4;
pub const HAS_BITWISE_BUILTIN: u32 = 1;
pub const HAS_DILUTED_POOL: u32 = 1;
pub const HAS_EC_OP_BUILTIN: u32 = 0;
pub const HAS_ECDSA_BUILTIN: u32 = 0;
pub const HAS_KECCAK_BUILTIN: u32 = 0;
pub const HAS_OUTPUT_BUILTIN: u32 = 1;
pub const HAS_PEDERSEN_BUILTIN: u32 = 1;
pub const HAS_POSEIDON_BUILTIN: u32 = 0;
pub const HAS_RANGE_CHECK_BUILTIN: u32 = 1;
pub const HAS_RANGE_CHECK96_BUILTIN: u32 = 0;
pub const IS_DYNAMIC_AIR: u32 = 0;
pub const LAYOUT_CODE: Felt = Felt::from_hex_unchecked("0x726563757273697665");
pub const LOG_CPU_COMPONENT_HEIGHT: u32 = 4;
pub const MASK_SIZE: u32 = 133;
pub const N_CONSTRAINTS: u32 = 93;
pub const N_DYNAMIC_PARAMS: u32 = 0;
pub const NUM_COLUMNS_FIRST: u32 = 7;
pub const NUM_COLUMNS_SECOND: u32 = 3;
pub const PEDERSEN_BUILTIN_RATIO: u32 = 128;
pub const PEDERSEN_BUILTIN_REPETITIONS: u32 = 1;
pub const PEDERSEN_BUILTIN_ROW_RATIO: u32 = 2048;
pub const PUBLIC_MEMORY_STEP: u32 = 16;
pub const RANGE_CHECK_BUILTIN_RATIO: u32 = 8;
pub const RANGE_CHECK_BUILTIN_ROW_RATIO: u32 = 128;
pub const RANGE_CHECK_N_PARTS: u32 = 8;

pub mod segments {
    pub const BITWISE: usize = 5;
    pub const N_SEGMENTS: usize = 6;
    pub const PEDERSEN: usize = 3;
    pub const RANGE_CHECK: usize = 4;
}

pub mod builtins {
    use starknet_crypto::Felt;

    pub const OUTPUT: Felt = Felt::from_hex_unchecked("0x6F7574707574");
    pub const PEDERSEN: Felt = Felt::from_hex_unchecked("0x706564657273656E");
    pub const RANGE_CHECK: Felt = Felt::from_hex_unchecked("0x72616E67655F636865636B");
    pub const BITWISE: Felt = Felt::from_hex_unchecked("0x62697477697365");
}

// Pedersen builtin
pub const SHIFT_POINT_X: Felt =
    Felt::from_hex_unchecked("0x49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");
pub const SHIFT_POINT_Y: Felt =
    Felt::from_hex_unchecked("0x3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a");

pub const BUILTINS: [Felt; 4] =
    [builtins::OUTPUT, builtins::PEDERSEN, builtins::RANGE_CHECK, builtins::BITWISE];

pub struct RecursiveLayout {}

impl LayoutTrait for RecursiveLayout {
    type InteractionElements = InteractionElements;
    fn eval_composition_polynomial(
        interaction_elements: &Self::InteractionElements,
        public_input: &PublicInput,
        mask_values: &[Felt],
        constraint_coefficients: &[Felt],
        point: &Felt,
        trace_domain_size: &Felt,
        trace_generator: &Felt,
    ) -> Result<Felt, CompositionPolyEvalError> {
        let memory_z = interaction_elements.memory_multi_column_perm_perm_interaction_elm;
        let memory_alpha = interaction_elements.memory_multi_column_perm_hash_interaction_elm0;

        // Public memory
        let public_memory_column_size = trace_domain_size
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(PUBLIC_MEMORY_STEP)));
        assert!(public_memory_column_size < u128::MAX.into());
        let public_memory_prod_ratio = public_input.get_public_memory_product_ratio(
            memory_z,
            memory_alpha,
            public_memory_column_size,
        );

        // Diluted
        let diluted_z = interaction_elements.diluted_check_interaction_z;
        let diluted_alpha = interaction_elements.diluted_check_interaction_alpha;
        let diluted_prod = get_diluted_product(
            DILUTED_N_BITS.into(),
            DILUTED_SPACING.into(),
            diluted_z,
            diluted_alpha,
        );

        // Periodic columns
        let n_steps = Felt::TWO.pow_felt(&public_input.log_n_steps);
        let n_pedersen_hash_copies = n_steps.field_div(&NonZeroFelt::from_felt_unchecked(
            Felt::from(PEDERSEN_BUILTIN_RATIO * PEDERSEN_BUILTIN_REPETITIONS),
        ));
        assert!(n_pedersen_hash_copies < u128::MAX.into());
        let pedersen_point = point.pow_felt(&n_pedersen_hash_copies);
        let pedersen_points_x = eval_pedersen_x(pedersen_point);
        let pedersen_points_y = eval_pedersen_y(pedersen_point);

        let global_values = GlobalValues {
            trace_length: *trace_domain_size,
            initial_pc: public_input
                .segments
                .get(crate::layout::segments::PROGRAM)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::PROGRAM,
                })?
                .begin_addr,
            final_pc: public_input
                .segments
                .get(crate::layout::segments::PROGRAM)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::PROGRAM,
                })?
                .stop_ptr,
            initial_ap: public_input
                .segments
                .get(crate::layout::segments::EXECUTION)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::EXECUTION,
                })?
                .begin_addr,
            final_ap: public_input
                .segments
                .get(crate::layout::segments::EXECUTION)
                .ok_or(CompositionPolyEvalError::SegmentMissing {
                    segment: crate::layout::segments::EXECUTION,
                })?
                .stop_ptr,
            initial_pedersen_addr: public_input
                .segments
                .get(segments::PEDERSEN)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::PEDERSEN })?
                .begin_addr,
            initial_range_check_addr: public_input
                .segments
                .get(segments::RANGE_CHECK)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::RANGE_CHECK })?
                .begin_addr,
            initial_bitwise_addr: public_input
                .segments
                .get(segments::BITWISE)
                .ok_or(CompositionPolyEvalError::SegmentMissing { segment: segments::BITWISE })?
                .begin_addr,
            range_check_min: public_input.range_check_min,
            range_check_max: public_input.range_check_max,
            offset_size: Felt::from(0x10000),     // 2**16
            half_offset_size: Felt::from(0x8000), // 2**15
            pedersen_shift_point: EcPoint { x: SHIFT_POINT_X, y: SHIFT_POINT_Y },
            pedersen_points_x,
            pedersen_points_y,
            memory_multi_column_perm_perm_interaction_elm: memory_z,
            memory_multi_column_perm_hash_interaction_elm0: memory_alpha,
            range_check16_perm_interaction_elm: interaction_elements
                .range_check16_perm_interaction_elm,
            diluted_check_permutation_interaction_elm: interaction_elements
                .diluted_check_permutation_interaction_elm,
            diluted_check_interaction_z: diluted_z,
            diluted_check_interaction_alpha: diluted_alpha,
            memory_multi_column_perm_perm_public_memory_prod: public_memory_prod_ratio,
            range_check16_perm_public_memory_prod: Felt::from(1),
            diluted_check_first_elm: Felt::from(0),
            diluted_check_permutation_public_memory_prod: Felt::from(1),
            diluted_check_final_cum_val: diluted_prod,
        };

        Ok(autogenerated::eval_composition_polynomial_inner(
            mask_values,
            constraint_coefficients,
            point,
            trace_generator,
            &global_values,
        ))
    }
    fn eval_oods_polynomial(
        column_values: &[Felt],
        oods_values: &[Felt],
        constraint_coefficients: &[Felt],
        point: &Felt,
        oods_point: &Felt,
        trace_generator: &Felt,
    ) -> Felt {
        autogenerated::eval_oods_polynomial_inner(
            column_values,
            oods_values,
            constraint_coefficients,
            point,
            oods_point,
            trace_generator,
        )
    }
    fn traces_commit(
        transcript: &mut cairovm_verifier_transcript::transcript::Transcript,
        unsent_commitment: &crate::trace::UnsentCommitment,
        config: crate::trace::config::Config,
    ) -> crate::trace::Commitment<Self::InteractionElements> {
        // Read original commitment.
        let original_commitment =
            table_commit(transcript, unsent_commitment.original, config.original);

        // Generate interaction elements for the first interaction.
        let interaction_elements = Self::InteractionElements::new(transcript);

        // Read interaction commitment.
        let interaction_commitment =
            table_commit(transcript, unsent_commitment.interaction, config.interaction);

        crate::trace::Commitment {
            original: original_commitment,
            interaction_elements,
            interaction: interaction_commitment,
        }
    }
    fn traces_decommit(
        queries: &[Felt],
        commitment: crate::trace::Commitment<Self::InteractionElements>,
        decommitment: crate::trace::Decommitment,
        witness: crate::trace::Witness,
    ) -> Result<(), crate::trace::decommit::Error> {
        Ok(table_decommit(commitment.original, queries, decommitment.original, witness.original)
            .and(table_decommit(
                commitment.interaction,
                queries,
                decommitment.interaction,
                witness.interaction,
            ))?)
    }
    fn validate_public_input(
        public_input: &PublicInput,
        stark_domains: &crate::domains::StarkDomains,
    ) -> Result<(), PublicInputError> {
        ensure!(public_input.log_n_steps < MAX_LOG_N_STEPS, PublicInputError::MaxSteps);

        let n_steps = Felt::TWO.pow_felt(&public_input.log_n_steps);
        let trace_length = stark_domains.trace_domain_size;
        ensure!(
            n_steps * Felt::from(CPU_COMPONENT_HEIGHT) * Felt::from(CPU_COMPONENT_STEP)
                == trace_length,
            PublicInputError::TraceLengthInvalid
        );

        ensure!(Felt::ZERO <= public_input.range_check_min, PublicInputError::RangeCheckInvalid);
        ensure!(
            public_input.range_check_min < public_input.range_check_max,
            PublicInputError::RangeCheckInvalid
        );
        ensure!(
            public_input.range_check_max <= MAX_RANGE_CHECK,
            PublicInputError::RangeCheckInvalid
        );

        ensure!(public_input.layout == LAYOUT_CODE, PublicInputError::LayoutCodeInvalid);

        let output_uses = public_input
            .segments
            .get(crate::layout::segments::OUTPUT)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr
            - public_input
                .segments
                .get(crate::layout::segments::OUTPUT)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr;
        ensure!(output_uses < u128::MAX.into(), PublicInputError::UsesInvalid);

        let pedersen_copies = trace_length
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(PEDERSEN_BUILTIN_ROW_RATIO)));
        let pedersen_uses = (public_input
            .segments
            .get(segments::PEDERSEN)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr
            - public_input
                .segments
                .get(segments::PEDERSEN)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr)
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::THREE));
        ensure!(pedersen_uses < pedersen_copies, PublicInputError::UsesInvalid);

        let range_check_copies = trace_length.field_div(&NonZeroFelt::from_felt_unchecked(
            Felt::from(RANGE_CHECK_BUILTIN_ROW_RATIO),
        ));
        let range_check_uses = public_input
            .segments
            .get(segments::RANGE_CHECK)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr
            - public_input
                .segments
                .get(segments::RANGE_CHECK)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr;
        ensure!(range_check_uses < range_check_copies, PublicInputError::UsesInvalid);

        let bitwise_copies = trace_length
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(BITWISE_ROW_RATIO)));
        let bitwise_uses = (public_input
            .segments
            .get(segments::BITWISE)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::OUTPUT })?
            .stop_ptr
            - public_input
                .segments
                .get(segments::BITWISE)
                .ok_or(PublicInputError::SegmentMissing {
                    segment: crate::layout::segments::OUTPUT,
                })?
                .begin_addr)
            .field_div(&NonZeroFelt::from_felt_unchecked(Felt::from(0x5)));
        ensure!(bitwise_uses < bitwise_copies, PublicInputError::UsesInvalid);
        Ok(())
    }

    fn verify_public_input(public_input: &PublicInput) -> Result<(Felt, Felt), PublicInputError> {
        let public_segments = &public_input.segments;

        let initial_pc = public_segments
            .get(crate::layout::segments::PROGRAM)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .begin_addr;
        let final_pc = public_segments
            .get(crate::layout::segments::PROGRAM)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .stop_ptr;
        let initial_ap = public_segments
            .get(crate::layout::segments::EXECUTION)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .begin_addr;
        let initial_fp = initial_ap;
        let final_ap = public_segments
            .get(crate::layout::segments::EXECUTION)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .stop_ptr;
        let output_start = public_segments
            .get(crate::layout::segments::OUTPUT)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .begin_addr;
        let output_stop = public_segments
            .get(crate::layout::segments::OUTPUT)
            .ok_or(PublicInputError::SegmentMissing { segment: crate::layout::segments::PROGRAM })?
            .stop_ptr;

        ensure!(initial_ap < MAX_ADDRESS, PublicInputError::MaxSteps);
        ensure!(final_ap < MAX_ADDRESS, PublicInputError::MaxSteps);

        // TODO support more pages?
        ensure!(public_input.continuous_page_headers.is_empty(), PublicInputError::MaxSteps);

        let memory = &public_input
            .main_page
            .iter()
            .flat_map(|v| vec![v.address, v.value])
            .collect::<Vec<Felt>>();

        // 1. Program segment
        ensure!(initial_pc == INITIAL_PC, PublicInputError::MaxSteps);
        ensure!(final_pc == INITIAL_PC + 4, PublicInputError::MaxSteps);

        let program_end_pc = initial_fp - 2;
        let program = &memory[initial_pc.to_bigint().try_into().unwrap()
            ..program_end_pc.to_bigint().try_into().unwrap()];

        let program_hash = poseidon_hash_many(program);

        let output_len: usize = (output_stop - output_start).to_bigint().try_into().unwrap();
        // 3. Output segment
        let output = &memory[memory.len() - output_len..memory.len()];
        let output_hash = poseidon_hash_many(output);

        Ok((program_hash, output_hash))
    }
}
