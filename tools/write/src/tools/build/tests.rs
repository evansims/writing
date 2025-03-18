#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::build::lazy_cache::LazyBuildCache;
    use crate::tools::build::cache::BuildCache;
    use std::path::PathBuf;
    use std::time::{Duration, Instant};
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    /// Test that the lazy cache works correctly
    #[test]
    fn test_lazy_cache_basic() {
        // Create a temporary directory for our test
        let temp_dir = tempdir().unwrap();
        let cache_path = temp_dir.path().join(".write-cache/build.json");

        // Ensure the cache directory exists
        fs::create_dir_all(cache_path.parent().unwrap()).unwrap();

        // Create a mock cache file
        let cache = BuildCache::new();
        let cache_content = serde_json::to_string_pretty(&cache).unwrap();
        fs::write(&cache_path, cache_content).unwrap();

        // Create a lazy cache
        let lazy_cache = LazyBuildCache::new()
            .with_invalidation_timeout(Duration::from_secs(10));

        // Test that we can get the cache
        let cache_guard = lazy_cache.get().unwrap();
        assert!(cache_guard.is_some());

        // Test clearing the cache
        lazy_cache.clear().unwrap();
        let cache_guard = lazy_cache.get().unwrap();
        assert!(cache_guard.is_some());

        // Test reloading the cache
        lazy_cache.reload().unwrap();
        let cache_guard = lazy_cache.get().unwrap();
        assert!(cache_guard.is_some());
    }

    /// Test that the lazy cache properly tracks file modifications
    #[test]
    fn test_lazy_cache_file_tracking() {
        // Create a temporary directory for our test
        let temp_dir = tempdir().unwrap();
        let cache_path = temp_dir.path().join(".write-cache/build.json");
        let test_file_path = temp_dir.path().join("test_file.txt");

        // Create a test file
        fs::write(&test_file_path, "test content").unwrap();

        // Create a lazy cache
        let lazy_cache = LazyBuildCache::new();

        // Check if the file needs rebuilding (should return true since it's not in the cache)
        let needs_rebuild = lazy_cache.needs_rebuild(&test_file_path).unwrap();
        assert!(needs_rebuild);

        // Add the file to the cache
        let output_file = temp_dir.path().join("output.html");
        lazy_cache.add_file(test_file_path.clone(), vec![output_file.clone()]).unwrap();

        // Check if the file needs rebuilding now (should return false)
        let needs_rebuild = lazy_cache.needs_rebuild(&test_file_path).unwrap();
        assert!(!needs_rebuild);

        // Modify the file
        std::thread::sleep(Duration::from_millis(10)); // Ensure the modification time is different
        let mut file = fs::OpenOptions::new().write(true).open(&test_file_path).unwrap();
        file.write_all("modified content".as_bytes()).unwrap();
        drop(file);

        // Check if the file needs rebuilding now (should return true)
        let needs_rebuild = lazy_cache.needs_rebuild(&test_file_path).unwrap();
        assert!(needs_rebuild);
    }

    /// Benchmark lazy loading vs. direct loading
    #[test]
    fn benchmark_lazy_loading() {
        // Skip this test in CI/CD environments or when not running benchmarks
        if std::env::var("CI").is_ok() || !std::env::var("RUN_BENCHMARKS").is_ok() {
            return;
        }

        // Create a temporary directory for our test
        let temp_dir = tempdir().unwrap();
        let cache_path = temp_dir.path().join(".write-cache/build.json");

        // Ensure the cache directory exists
        fs::create_dir_all(cache_path.parent().unwrap()).unwrap();

        // Create a mock cache with many files
        let mut cache = BuildCache::new();
        for i in 0..1000 {
            let file_path = PathBuf::from(format!("file_{}.md", i));
            let outputs = vec![PathBuf::from(format!("file_{}.html", i))];
            cache.add_file(file_path, outputs).unwrap();
        }

        // Save the cache
        let cache_content = serde_json::to_string_pretty(&cache).unwrap();
        fs::write(&cache_path, cache_content).unwrap();

        // Benchmark direct loading
        let iterations = 100;
        let direct_start = Instant::now();
        for _ in 0..iterations {
            let _cache = cache::BuildCache::load(&cache_path).unwrap();
        }
        let direct_elapsed = direct_start.elapsed();

        // Benchmark lazy loading
        let lazy_cache = LazyBuildCache::new()
            .with_invalidation_timeout(Duration::from_millis(1)); // Short timeout to force reloads

        let lazy_start = Instant::now();
        for _ in 0..iterations {
            let _cache_guard = lazy_cache.get().unwrap();
        }
        let lazy_elapsed = lazy_start.elapsed();

        // Print benchmark results
        println!("Direct loading: {:?} for {} iterations", direct_elapsed, iterations);
        println!("Lazy loading: {:?} for {} iterations", lazy_elapsed, iterations);
        println!("Lazy loading is {:.2}x faster", direct_elapsed.as_secs_f64() / lazy_elapsed.as_secs_f64());

        // Benchmark cached access
        lazy_cache.reload().unwrap(); // Reset the cache

        let cached_start = Instant::now();
        for _ in 0..iterations {
            let _cache_guard = lazy_cache.get().unwrap();
        }
        let cached_elapsed = cached_start.elapsed();

        println!("Cached access: {:?} for {} iterations", cached_elapsed, iterations);
        println!("Cached access is {:.2}x faster than direct loading", direct_elapsed.as_secs_f64() / cached_elapsed.as_secs_f64());
    }

    /// Test the singleton function
    #[test]
    fn test_lazy_build_cache_singleton() {
        // Get the singleton instance
        let cache1 = crate::tools::build::lazy_build_cache();
        let cache2 = crate::tools::build::lazy_build_cache();

        // Both instances should be clones of the same underlying cache
        // We can't directly compare instances since LazyBuildCache doesn't implement PartialEq,
        // but we can test that modifying one affects the other

        // We'll use a temporary directory for this test
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test_file.txt");
        fs::write(&test_file, "test content").unwrap();

        // Add the file to the first cache instance
        cache1.add_file(test_file.clone(), vec![]).unwrap();

        // Check using the second instance that the file doesn't need rebuilding
        let needs_rebuild = cache2.needs_rebuild(&test_file).unwrap();
        assert!(!needs_rebuild);
    }
}