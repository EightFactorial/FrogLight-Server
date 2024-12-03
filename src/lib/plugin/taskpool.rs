use bevy::core::{TaskPoolOptions, TaskPoolThreadAssignmentPolicy};

/// The default [`TaskPoolOptions`] used by `FrogLight-Server`.
///
/// Assigns CPU cores as follows:
/// - 35% for `IO`, at least 1, no more than 8
/// - 25% for `async compute`, at least 1, no limit
/// - Remaining (~40%) for `compute`, at least 1, no limit
///
/// | CPU Cores/Threads | # IO | # Async Compute | # Compute |
/// |-------------------|------|-----------------|-----------|
/// | 1-3               | 1    | 1               | 1         |
/// | 4                 | 1    | 1               | 2         |
/// | 5                 | 2    | 1               | 2         |
/// | 6                 | 2    | 2               | 2         |
/// | 7                 | 2    | 2               | 3         |
/// | 8                 | 3    | 2               | 3         |
/// | 9                 | 3    | 2               | 4         |
/// | 10                | 4    | 3               | 3         |
/// | 11                | 4    | 3               | 4         |
/// | 12                | 4    | 3               | 5         |
/// | 13                | 5    | 3               | 5         |
/// | 14                | 5    | 4               | 5         |
/// | 15                | 5    | 4               | 6         |
/// | 16                | 6    | 4               | 6         |
/// | 24                | 8    | 6               | 10        |
/// | 32                | 8    | 8               | 16        |
/// | 48                | 8    | 12              | 28        |
/// | 64                | 8    | 16              | 40        |
/// | 128               | 8    | 32              | 88        |
pub const TASKPOOL_SETTINGS: TaskPoolOptions = TaskPoolOptions {
    // Use as many threads as possible
    min_total_threads: 1,
    max_total_threads: usize::MAX,

    // Assign threads based on Min/Max/Percent
    io: TaskPoolThreadAssignmentPolicy {
        min_threads: IO_MIN,
        max_threads: IO_MAX,
        percent: IO_PERCENT,
    },
    async_compute: TaskPoolThreadAssignmentPolicy {
        min_threads: ASYNC_COMPUTE_MIN,
        max_threads: ASYNC_COMPUTE_MAX,
        percent: ASYNC_COMPUTE_PERCENT,
    },
    compute: TaskPoolThreadAssignmentPolicy {
        min_threads: COMPUTE_MIN,
        max_threads: COMPUTE_MAX,
        percent: COMPUTE_PERCENT,
    },
};

// Use 35% of cores for IO, at least 1, no more than 8
const IO_MIN: usize = 1;
const IO_MAX: usize = 8;
const IO_PERCENT: f32 = 0.35;

// Use 25% of cores for async compute, at least 1, no limit
const ASYNC_COMPUTE_MIN: usize = 1;
const ASYNC_COMPUTE_MAX: usize = usize::MAX;
const ASYNC_COMPUTE_PERCENT: f32 = 0.25;

// Use all (~40%) remaining cores for compute, at least 1, no limit
const COMPUTE_MIN: usize = 1;
const COMPUTE_MAX: usize = usize::MAX;
const COMPUTE_PERCENT: f32 = 1.0;

#[cfg(test)]
mod tests {
    use bevy::core::{TaskPoolOptions, TaskPoolThreadAssignmentPolicy};

    /// The expected distribution of threads based on the number of cores.
    const EXPECTED_DISTRIBUTION: &[(usize, usize, usize, usize)] = &[
        (1, 1, 1, 1),
        (2, 1, 1, 1),
        (3, 1, 1, 1),
        (4, 1, 1, 2),
        (5, 2, 1, 2),
        (6, 2, 2, 2),
        (7, 2, 2, 3),
        (8, 3, 2, 3),
        (9, 3, 2, 4),
        (10, 4, 3, 3),
        (11, 4, 3, 4),
        (12, 4, 3, 5),
        (13, 5, 3, 5),
        (14, 5, 4, 5),
        (15, 5, 4, 6),
        (16, 6, 4, 6),
        (24, 8, 6, 10),
        (32, 8, 8, 16),
        (48, 8, 12, 28),
        (64, 8, 16, 40),
        (128, 8, 32, 88),
    ];

    /// Test the distribution of threads based on the number of cores.
    #[test]
    fn taskpool_threads() {
        for (cores, io, async_comp, comp) in EXPECTED_DISTRIBUTION {
            let expected = calculate_threads(*cores, &super::TASKPOOL_SETTINGS);
            assert_eq!(
                (*io, *async_comp, *comp),
                expected,
                "Cores: {cores}, Expected: ({io}, {async_comp}, {comp}), Actual: {expected:?}",
            );
        }
    }

    /// Calculate the number of threads to use based on the taskpool options and
    /// the number of cores.
    fn calculate_threads(cores: usize, options: &TaskPoolOptions) -> (usize, usize, usize) {
        let mut remaining = cores;

        // Calculate the number of threads for the IO pool
        let io = get_number_of_threads(&options.io, remaining, cores);
        remaining = remaining.saturating_sub(io);

        // Calculate the number of threads for the async compute pool
        let async_compute = get_number_of_threads(&options.async_compute, remaining, cores);
        remaining = remaining.saturating_sub(async_compute);

        // Calculate the number of threads for the compute pool
        let compute = get_number_of_threads(&options.compute, remaining, cores);

        (io, async_compute, compute)
    }

    /// Calculate the number of threads to use based on policy and remaining
    /// cores.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
    fn get_number_of_threads(
        policy: &TaskPoolThreadAssignmentPolicy,
        remaining: usize,
        total: usize,
    ) -> usize {
        let mut desired = (total as f32 * policy.percent).round() as usize;
        // Limit ourselves to the number of cores available
        desired = desired.min(remaining);
        // Clamp by min_threads, max_threads.
        desired.clamp(policy.min_threads, policy.max_threads)
    }
}
