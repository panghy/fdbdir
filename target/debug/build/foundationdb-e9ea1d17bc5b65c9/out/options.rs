use std::convert::TryFrom;
use crate::{FdbError, FdbResult};
use foundationdb_sys as fdb_sys;
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum NetworkOption {
    /// IP:PORT
    ///
    /// Deprecated
    LocalAddress(String),
    /// path to cluster file
    ///
    /// Deprecated
    ClusterFile(String),
    /// path to output directory (or NULL for current working directory)
    ///
    /// Enables trace output to a file in a directory of the clients choosing
    TraceEnable(String),
    /// max size of a single trace output file
    ///
    /// Sets the maximum size in bytes of a single trace output file. This value should be in the range ``[0, INT64_MAX]``. If the value is set to 0, there is no limit on individual file size. The default is a maximum size of 10,485,760 bytes.
    TraceRollSize(i32),
    /// max total size of trace files
    ///
    /// Sets the maximum size of all the trace output files put together. This value should be in the range ``[0, INT64_MAX]``. If the value is set to 0, there is no limit on the total size of the files. The default is a maximum size of 104,857,600 bytes. If the default roll size is used, this means that a maximum of 10 trace files will be written at a time.
    TraceMaxLogsSize(i32),
    /// value of the LogGroup attribute
    ///
    /// Sets the 'LogGroup' attribute with the specified value for all events in the trace output files. The default log group is 'default'.
    TraceLogGroup(String),
    /// Format of trace files
    ///
    /// Select the format of the log files. xml (the default) and json are supported.
    TraceFormat(String),
    /// Trace clock source
    ///
    /// Select clock source for trace files. now (the default) or realtime are supported.
    TraceClockSource(String),
    /// The identifier that will be part of all trace file names
    ///
    /// Once provided, this string will be used to replace the port/PID in the log file names.
    TraceFileIdentifier(String),
    /// Use the same base trace file name for all client threads as it did before version 7.2. The current default behavior is to use distinct trace file names for client threads by including their version and thread index.
    TraceShareAmongClientThreads,
    /// Initialize trace files on network setup, determine the local IP later. Otherwise tracing is initialized when opening the first database.
    TraceInitializeOnSetup,
    /// Append this suffix to partially written log files. When a log file is complete, it is renamed to remove the suffix. No separator is added between the file and the suffix. If you want to add a file extension, you should include the separator - e.g. '.tmp' instead of 'tmp' to add the 'tmp' extension.
    ///
    /// Set file suffix for partially written log files.
    TracePartialFileSuffix(String),
    /// knob_name=knob_value
    ///
    /// Set internal tuning or debugging knobs
    Knob(String),
    /// file path or linker-resolved name
    ///
    /// Deprecated
    TLSPlugin(String),
    /// certificates
    ///
    /// Set the certificate chain
    TLSCertBytes(Vec<u8>),
    /// file path
    ///
    /// Set the file from which to load the certificate chain
    TLSCertPath(String),
    /// key
    ///
    /// Set the private key corresponding to your own certificate
    TLSKeyBytes(Vec<u8>),
    /// file path
    ///
    /// Set the file from which to load the private key corresponding to your own certificate
    TLSKeyPath(String),
    /// verification pattern
    ///
    /// Set the peer certificate field verification criteria
    TLSVerifyPeers(Vec<u8>),
    BuggifyEnable,
    BuggifyDisable,
    /// probability expressed as a percentage between 0 and 100
    ///
    /// Set the probability of a BUGGIFY section being active for the current execution.  Only applies to code paths first traversed AFTER this option is changed.
    BuggifySectionActivatedProbability(i32),
    /// probability expressed as a percentage between 0 and 100
    ///
    /// Set the probability of an active BUGGIFY section being fired
    BuggifySectionFiredProbability(i32),
    /// ca bundle
    ///
    /// Set the ca bundle
    TLSCaBytes(Vec<u8>),
    /// file path
    ///
    /// Set the file from which to load the certificate authority bundle
    TLSCaPath(String),
    /// key passphrase
    ///
    /// Set the passphrase for encrypted private key. Password should be set before setting the key for the password to be used.
    TLSPassword(String),
    /// Disables the multi-version client API and instead uses the local client directly. Must be set before setting up the network.
    DisableMultiVersionClientApi,
    /// If set, callbacks from external client libraries can be called from threads created by the FoundationDB client library. Otherwise, callbacks will be called from either the thread used to add the callback or the network thread. Setting this option can improve performance when connected using an external client, but may not be safe to use in all environments. Must be set before setting up the network. WARNING: This feature is considered experimental at this time.
    CallbacksOnExternalThreads,
    /// path to client library
    ///
    /// Adds an external client library for use by the multi-version client API. Must be set before setting up the network.
    ExternalClientLibrary(String),
    /// path to directory containing client libraries
    ///
    /// Searches the specified path for dynamic libraries and adds them to the list of client libraries for use by the multi-version client API. Must be set before setting up the network.
    ExternalClientDirectory(String),
    /// Prevents connections through the local client, allowing only connections through externally loaded client libraries.
    DisableLocalClient,
    /// Number of client threads to be spawned.  Each cluster will be serviced by a single client thread.
    ///
    /// Spawns multiple worker threads for each version of the client that is loaded.  Setting this to a number greater than one implies disable_local_client.
    ClientThreadsPerVersion(i32),
    /// path to client library
    ///
    /// Adds an external client library to be used with a future version protocol. This option can be used testing purposes only!
    FutureVersionClientLibrary(String),
    /// Retain temporary external client library copies that are created for enabling multi-threading.
    RetainClientLibraryCopies,
    /// Ignore the failure to initialize some of the external clients
    IgnoreExternalClientFailures,
    /// Fail with an error if there is no client matching the server version the client is connecting to
    FailIncompatibleClient,
    /// Disables logging of client statistics, such as sampled transaction activity.
    DisableClientStatisticsLogging,
    /// Deprecated
    EnableSlowTaskProfiling,
    /// Enables debugging feature to perform run loop profiling. Requires trace logging to be enabled. WARNING: this feature is not recommended for use in production.
    EnableRunLoopProfiling,
    /// Prevents the multi-version client API from being disabled, even if no external clients are configured. This option is required to use GRV caching.
    DisableClientBypass,
    /// Enable client buggify - will make requests randomly fail (intended for client testing)
    ClientBuggifyEnable,
    /// Disable client buggify
    ClientBuggifyDisable,
    /// probability expressed as a percentage between 0 and 100
    ///
    /// Set the probability of a CLIENT_BUGGIFY section being active for the current execution.
    ClientBuggifySectionActivatedProbability(i32),
    /// probability expressed as a percentage between 0 and 100
    ///
    /// Set the probability of an active CLIENT_BUGGIFY section being fired. A section will only fire if it was activated
    ClientBuggifySectionFiredProbability(i32),
    /// Distributed tracer type. Choose from none, log_file, or network_lossy
    ///
    /// Set a tracer to run on the client. Should be set to the same value as the tracer set on the server.
    DistributedClientTracer(String),
    /// Client directory for temporary files. 
    ///
    /// Sets the directory for storing temporary files created by FDB client, such as temporary copies of client libraries. Defaults to /tmp
    ClientTmpDir(String),
}
impl NetworkOption {
    pub fn code(&self) -> fdb_sys::FDBNetworkOption {
        match *self {
            NetworkOption::LocalAddress(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_LOCAL_ADDRESS,
            NetworkOption::ClusterFile(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLUSTER_FILE,
            NetworkOption::TraceEnable(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_ENABLE,
            NetworkOption::TraceRollSize(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_ROLL_SIZE,
            NetworkOption::TraceMaxLogsSize(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_MAX_LOGS_SIZE,
            NetworkOption::TraceLogGroup(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_LOG_GROUP,
            NetworkOption::TraceFormat(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_FORMAT,
            NetworkOption::TraceClockSource(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_CLOCK_SOURCE,
            NetworkOption::TraceFileIdentifier(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_FILE_IDENTIFIER,
            NetworkOption::TraceShareAmongClientThreads => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_SHARE_AMONG_CLIENT_THREADS,
            NetworkOption::TraceInitializeOnSetup => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_INITIALIZE_ON_SETUP,
            NetworkOption::TracePartialFileSuffix(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TRACE_PARTIAL_FILE_SUFFIX,
            NetworkOption::Knob(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_KNOB,
            NetworkOption::TLSPlugin(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_PLUGIN,
            NetworkOption::TLSCertBytes(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_CERT_BYTES,
            NetworkOption::TLSCertPath(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_CERT_PATH,
            NetworkOption::TLSKeyBytes(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_KEY_BYTES,
            NetworkOption::TLSKeyPath(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_KEY_PATH,
            NetworkOption::TLSVerifyPeers(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_VERIFY_PEERS,
            NetworkOption::BuggifyEnable => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_BUGGIFY_ENABLE,
            NetworkOption::BuggifyDisable => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_BUGGIFY_DISABLE,
            NetworkOption::BuggifySectionActivatedProbability(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_BUGGIFY_SECTION_ACTIVATED_PROBABILITY,
            NetworkOption::BuggifySectionFiredProbability(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_BUGGIFY_SECTION_FIRED_PROBABILITY,
            NetworkOption::TLSCaBytes(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_CA_BYTES,
            NetworkOption::TLSCaPath(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_CA_PATH,
            NetworkOption::TLSPassword(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_TLS_PASSWORD,
            NetworkOption::DisableMultiVersionClientApi => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_DISABLE_MULTI_VERSION_CLIENT_API,
            NetworkOption::CallbacksOnExternalThreads => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CALLBACKS_ON_EXTERNAL_THREADS,
            NetworkOption::ExternalClientLibrary(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_EXTERNAL_CLIENT_LIBRARY,
            NetworkOption::ExternalClientDirectory(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_EXTERNAL_CLIENT_DIRECTORY,
            NetworkOption::DisableLocalClient => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_DISABLE_LOCAL_CLIENT,
            NetworkOption::ClientThreadsPerVersion(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLIENT_THREADS_PER_VERSION,
            NetworkOption::FutureVersionClientLibrary(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_FUTURE_VERSION_CLIENT_LIBRARY,
            NetworkOption::RetainClientLibraryCopies => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_RETAIN_CLIENT_LIBRARY_COPIES,
            NetworkOption::IgnoreExternalClientFailures => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_IGNORE_EXTERNAL_CLIENT_FAILURES,
            NetworkOption::FailIncompatibleClient => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_FAIL_INCOMPATIBLE_CLIENT,
            NetworkOption::DisableClientStatisticsLogging => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_DISABLE_CLIENT_STATISTICS_LOGGING,
            NetworkOption::EnableSlowTaskProfiling => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_ENABLE_SLOW_TASK_PROFILING,
            NetworkOption::EnableRunLoopProfiling => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_ENABLE_RUN_LOOP_PROFILING,
            NetworkOption::DisableClientBypass => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_DISABLE_CLIENT_BYPASS,
            NetworkOption::ClientBuggifyEnable => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLIENT_BUGGIFY_ENABLE,
            NetworkOption::ClientBuggifyDisable => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLIENT_BUGGIFY_DISABLE,
            NetworkOption::ClientBuggifySectionActivatedProbability(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLIENT_BUGGIFY_SECTION_ACTIVATED_PROBABILITY,
            NetworkOption::ClientBuggifySectionFiredProbability(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLIENT_BUGGIFY_SECTION_FIRED_PROBABILITY,
            NetworkOption::DistributedClientTracer(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_DISTRIBUTED_CLIENT_TRACER,
            NetworkOption::ClientTmpDir(..) => fdb_sys::FDBNetworkOption_FDB_NET_OPTION_CLIENT_TMP_DIR,
        }
    }
    pub unsafe fn apply(&self) -> FdbResult<()> {
        let code = self.code();
        let err = match *self {
            NetworkOption::LocalAddress(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::ClusterFile(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TraceEnable(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TraceRollSize(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::TraceMaxLogsSize(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::TraceLogGroup(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TraceFormat(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TraceClockSource(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TraceFileIdentifier(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TraceShareAmongClientThreads => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::TraceInitializeOnSetup => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::TracePartialFileSuffix(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::Knob(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSPlugin(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSCertBytes(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSCertPath(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSKeyBytes(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSKeyPath(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSVerifyPeers(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::BuggifyEnable => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::BuggifyDisable => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::BuggifySectionActivatedProbability(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::BuggifySectionFiredProbability(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::TLSCaBytes(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSCaPath(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::TLSPassword(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::DisableMultiVersionClientApi => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::CallbacksOnExternalThreads => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::ExternalClientLibrary(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::ExternalClientDirectory(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::DisableLocalClient => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::ClientThreadsPerVersion(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::FutureVersionClientLibrary(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::RetainClientLibraryCopies => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::IgnoreExternalClientFailures => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::FailIncompatibleClient => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::DisableClientStatisticsLogging => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::EnableSlowTaskProfiling => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::EnableRunLoopProfiling => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::DisableClientBypass => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::ClientBuggifyEnable => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::ClientBuggifyDisable => fdb_sys::fdb_network_set_option(code, std::ptr::null(), 0),
            NetworkOption::ClientBuggifySectionActivatedProbability(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::ClientBuggifySectionFiredProbability(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_network_set_option(code, data.as_ptr() as *const u8, 8)
            }
            NetworkOption::DistributedClientTracer(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            NetworkOption::ClientTmpDir(ref v) => {
                fdb_sys::fdb_network_set_option(code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
        };
        if err != 0 { Err(FdbError::from_code(err)) } else { Ok(()) }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DatabaseOption {
    /// Max location cache entries
    ///
    /// Set the size of the client location cache. Raising this value can boost performance in very large databases where clients access data in a near-random pattern. Defaults to 100000.
    LocationCacheSize(i32),
    /// Max outstanding watches
    ///
    /// Set the maximum number of watches allowed to be outstanding on a database connection. Increasing this number could result in increased resource usage. Reducing this number will not cancel any outstanding watches. Defaults to 10000 and cannot be larger than 1000000.
    MaxWatches(i32),
    /// Hexadecimal ID
    ///
    /// Specify the machine ID that was passed to fdbserver processes running on the same machine as this client, for better location-aware load balancing.
    MachineId(String),
    /// Hexadecimal ID
    ///
    /// Specify the datacenter ID that was passed to fdbserver processes running in the same datacenter as this client, for better location-aware load balancing.
    DatacenterId(String),
    /// Snapshot read operations will see the results of writes done in the same transaction. This is the default behavior.
    SnapshotRywEnable,
    /// Snapshot read operations will not see the results of writes done in the same transaction. This was the default behavior prior to API version 300.
    SnapshotRywDisable,
    /// Maximum length of escaped key and value fields.
    ///
    /// Sets the maximum escaped length of key and value fields to be logged to the trace file via the LOG_TRANSACTION option. This sets the ``transaction_logging_max_field_length`` option of each transaction created by this database. See the transaction option description for more information.
    TransactionLoggingMaxFieldLength(i32),
    /// value in milliseconds of timeout
    ///
    /// Set a timeout in milliseconds which, when elapsed, will cause each transaction automatically to be cancelled. This sets the ``timeout`` option of each transaction created by this database. See the transaction option description for more information. Using this option requires that the API version is 610 or higher.
    TransactionTimeout(i32),
    /// number of times to retry
    ///
    /// Set a maximum number of retries after which additional calls to ``onError`` will throw the most recently seen error code. This sets the ``retry_limit`` option of each transaction created by this database. See the transaction option description for more information.
    TransactionRetryLimit(i32),
    /// value in milliseconds of maximum delay
    ///
    /// Set the maximum amount of backoff delay incurred in the call to ``onError`` if the error is retryable. This sets the ``max_retry_delay`` option of each transaction created by this database. See the transaction option description for more information.
    TransactionMaxRetryDelay(i32),
    /// value in bytes
    ///
    /// Set the maximum transaction size in bytes. This sets the ``size_limit`` option on each transaction created by this database. See the transaction option description for more information.
    TransactionSizeLimit(i32),
    /// The read version will be committed, and usually will be the latest committed, but might not be the latest committed in the event of a simultaneous fault and misbehaving clock.
    TransactionCausalReadRisky,
    /// Deprecated. Addresses returned by get_addresses_for_key include the port when enabled. As of api version 630, this option is enabled by default and setting this has no effect.
    TransactionIncludePortInAddress,
    /// Set a random idempotency id for all transactions. See the transaction option description for more information. This feature is in development and not ready for general use.
    TransactionAutomaticIdempotency,
    /// Allows ``get`` operations to read from sections of keyspace that have become unreadable because of versionstamp operations. This sets the ``bypass_unreadable`` option of each transaction created by this database. See the transaction option description for more information.
    TransactionBypassUnreadable,
    /// By default, operations that are performed on a transaction while it is being committed will not only fail themselves, but they will attempt to fail other in-flight operations (such as the commit) as well. This behavior is intended to help developers discover situations where operations could be unintentionally executed after the transaction has been reset. Setting this option removes that protection, causing only the offending operation to fail.
    TransactionUsedDuringCommitProtectionDisable,
    /// Enables conflicting key reporting on all transactions, allowing them to retrieve the keys that are conflicting with other transactions.
    TransactionReportConflictingKeys,
    /// Use configuration database.
    UseConfigDatabase,
    /// integer between 0 and 100 expressing the probability a client will verify it can't read stale data
    ///
    /// Enables verification of causal read risky by checking whether clients are able to read stale data when they detect a recovery, and logging an error if so.
    TestCausalReadRisky(i32),
}
impl DatabaseOption {
    pub fn code(&self) -> fdb_sys::FDBDatabaseOption {
        match *self {
            DatabaseOption::LocationCacheSize(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_LOCATION_CACHE_SIZE,
            DatabaseOption::MaxWatches(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_MAX_WATCHES,
            DatabaseOption::MachineId(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_MACHINE_ID,
            DatabaseOption::DatacenterId(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_DATACENTER_ID,
            DatabaseOption::SnapshotRywEnable => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_SNAPSHOT_RYW_ENABLE,
            DatabaseOption::SnapshotRywDisable => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_SNAPSHOT_RYW_DISABLE,
            DatabaseOption::TransactionLoggingMaxFieldLength(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_LOGGING_MAX_FIELD_LENGTH,
            DatabaseOption::TransactionTimeout(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_TIMEOUT,
            DatabaseOption::TransactionRetryLimit(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_RETRY_LIMIT,
            DatabaseOption::TransactionMaxRetryDelay(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_MAX_RETRY_DELAY,
            DatabaseOption::TransactionSizeLimit(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_SIZE_LIMIT,
            DatabaseOption::TransactionCausalReadRisky => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_CAUSAL_READ_RISKY,
            DatabaseOption::TransactionIncludePortInAddress => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_INCLUDE_PORT_IN_ADDRESS,
            DatabaseOption::TransactionAutomaticIdempotency => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_AUTOMATIC_IDEMPOTENCY,
            DatabaseOption::TransactionBypassUnreadable => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_BYPASS_UNREADABLE,
            DatabaseOption::TransactionUsedDuringCommitProtectionDisable => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_USED_DURING_COMMIT_PROTECTION_DISABLE,
            DatabaseOption::TransactionReportConflictingKeys => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TRANSACTION_REPORT_CONFLICTING_KEYS,
            DatabaseOption::UseConfigDatabase => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_USE_CONFIG_DATABASE,
            DatabaseOption::TestCausalReadRisky(..) => fdb_sys::FDBDatabaseOption_FDB_DB_OPTION_TEST_CAUSAL_READ_RISKY,
        }
    }
    pub unsafe fn apply(&self, target: *mut fdb_sys::FDBDatabase) -> FdbResult<()> {
        let code = self.code();
        let err = match *self {
            DatabaseOption::LocationCacheSize(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::MaxWatches(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::MachineId(ref v) => {
                fdb_sys::fdb_database_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            DatabaseOption::DatacenterId(ref v) => {
                fdb_sys::fdb_database_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            DatabaseOption::SnapshotRywEnable => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::SnapshotRywDisable => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TransactionLoggingMaxFieldLength(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::TransactionTimeout(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::TransactionRetryLimit(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::TransactionMaxRetryDelay(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::TransactionSizeLimit(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            DatabaseOption::TransactionCausalReadRisky => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TransactionIncludePortInAddress => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TransactionAutomaticIdempotency => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TransactionBypassUnreadable => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TransactionUsedDuringCommitProtectionDisable => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TransactionReportConflictingKeys => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::UseConfigDatabase => fdb_sys::fdb_database_set_option(target, code, std::ptr::null(), 0),
            DatabaseOption::TestCausalReadRisky(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_database_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
        };
        if err != 0 { Err(FdbError::from_code(err)) } else { Ok(()) }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TransactionOption {
    /// The transaction, if not self-conflicting, may be committed a second time after commit succeeds, in the event of a fault
    CausalWriteRisky,
    /// The read version will be committed, and usually will be the latest committed, but might not be the latest committed in the event of a simultaneous fault and misbehaving clock.
    CausalReadRisky,
    CausalReadDisable,
    /// Addresses returned by get_addresses_for_key include the port when enabled. As of api version 630, this option is enabled by default and setting this has no effect.
    IncludePortInAddress,
    /// The next write performed on this transaction will not generate a write conflict range. As a result, other transactions which read the key(s) being modified by the next write will not conflict with this transaction. Care needs to be taken when using this option on a transaction that is shared between multiple threads. When setting this option, write conflict ranges will be disabled on the next write operation, regardless of what thread it is on.
    NextWriteNoWriteConflictRange,
    /// Reads performed by a transaction will not see any prior mutations that occured in that transaction, instead seeing the value which was in the database at the transaction's read version. This option may provide a small performance benefit for the client, but also disables a number of client-side optimizations which are beneficial for transactions which tend to read and write the same keys within a single transaction. It is an error to set this option after performing any reads or writes on the transaction.
    ReadYourWritesDisable,
    /// Deprecated
    ReadAheadDisable,
    /// Storage server should cache disk blocks needed for subsequent read requests in this transaction.  This is the default behavior.
    ReadServerSideCacheEnable,
    /// Storage server should not cache disk blocks needed for subsequent read requests in this transaction.  This can be used to avoid cache pollution for reads not expected to be repeated.
    ReadServerSideCacheDisable,
    /// Use normal read priority for subsequent read requests in this transaction.  This is the default read priority.
    ReadPriorityNormal,
    /// Use low read priority for subsequent read requests in this transaction.
    ReadPriorityLow,
    /// Use high read priority for subsequent read requests in this transaction.
    ReadPriorityHigh,
    DurabilityDatacenter,
    DurabilityRisky,
    /// Deprecated
    DurabilityDevNullIsWebScale,
    /// Specifies that this transaction should be treated as highest priority and that lower priority transactions should block behind this one. Use is discouraged outside of low-level tools
    PrioritySystemImmediate,
    /// Specifies that this transaction should be treated as low priority and that default priority transactions will be processed first. Batch priority transactions will also be throttled at load levels smaller than for other types of transactions and may be fully cut off in the event of machine failures. Useful for doing batch work simultaneously with latency-sensitive work
    PriorityBatch,
    /// This is a write-only transaction which sets the initial configuration. This option is designed for use by database system tools only.
    InitializeNewDatabase,
    /// Allows this transaction to read and modify system keys (those that start with the byte 0xFF). Implies raw_access.
    AccessSystemKeys,
    /// Allows this transaction to read system keys (those that start with the byte 0xFF). Implies raw_access.
    ReadSystemKeys,
    /// Allows this transaction to access the raw key-space when tenant mode is on.
    RawAccess,
    /// Allows this transaction to bypass storage quota enforcement. Should only be used for transactions that directly or indirectly decrease the size of the tenant group's data.
    BypassStorageQuota,
    /// Optional transaction name
    ///
    DebugRetryLogging(String),
    /// String identifier to be used in the logs when tracing this transaction. The identifier must not exceed 100 characters.
    ///
    /// Deprecated
    TransactionLoggingEnable(String),
    /// String identifier to be used when tracing or profiling this transaction. The identifier must not exceed 100 characters.
    ///
    /// Sets a client provided identifier for the transaction that will be used in scenarios like tracing or profiling. Client trace logging or transaction profiling must be separately enabled.
    DebugTransactionIdentifier(String),
    /// Enables tracing for this transaction and logs results to the client trace logs. The DEBUG_TRANSACTION_IDENTIFIER option must be set before using this option, and client trace logging must be enabled to get log output.
    LogTransaction,
    /// Maximum length of escaped key and value fields.
    ///
    /// Sets the maximum escaped length of key and value fields to be logged to the trace file via the LOG_TRANSACTION option, after which the field will be truncated. A negative value disables truncation.
    TransactionLoggingMaxFieldLength(i32),
    /// Sets an identifier for server tracing of this transaction. When committed, this identifier triggers logging when each part of the transaction authority encounters it, which is helpful in diagnosing slowness in misbehaving clusters. The identifier is randomly generated. When there is also a debug_transaction_identifier, both IDs are logged together.
    ServerRequestTracing,
    /// value in milliseconds of timeout
    ///
    /// Set a timeout in milliseconds which, when elapsed, will cause the transaction automatically to be cancelled. Valid parameter values are ``[0, INT_MAX]``. If set to 0, will disable all timeouts. All pending and any future uses of the transaction will throw an exception. The transaction can be used again after it is reset. Prior to API version 610, like all other transaction options, the timeout must be reset after a call to ``onError``. If the API version is 610 or greater, the timeout is not reset after an ``onError`` call. This allows the user to specify a longer timeout on specific transactions than the default timeout specified through the ``transaction_timeout`` database option without the shorter database timeout cancelling transactions that encounter a retryable error. Note that at all API versions, it is safe and legal to set the timeout each time the transaction begins, so most code written assuming the older behavior can be upgraded to the newer behavior without requiring any modification, and the caller is not required to implement special logic in retry loops to only conditionally set this option.
    Timeout(i32),
    /// number of times to retry
    ///
    /// Set a maximum number of retries after which additional calls to ``onError`` will throw the most recently seen error code. Valid parameter values are ``[-1, INT_MAX]``. If set to -1, will disable the retry limit. Prior to API version 610, like all other transaction options, the retry limit must be reset after a call to ``onError``. If the API version is 610 or greater, the retry limit is not reset after an ``onError`` call. Note that at all API versions, it is safe and legal to set the retry limit each time the transaction begins, so most code written assuming the older behavior can be upgraded to the newer behavior without requiring any modification, and the caller is not required to implement special logic in retry loops to only conditionally set this option.
    RetryLimit(i32),
    /// value in milliseconds of maximum delay
    ///
    /// Set the maximum amount of backoff delay incurred in the call to ``onError`` if the error is retryable. Defaults to 1000 ms. Valid parameter values are ``[0, INT_MAX]``. If the maximum retry delay is less than the current retry delay of the transaction, then the current retry delay will be clamped to the maximum retry delay. Prior to API version 610, like all other transaction options, the maximum retry delay must be reset after a call to ``onError``. If the API version is 610 or greater, the retry limit is not reset after an ``onError`` call. Note that at all API versions, it is safe and legal to set the maximum retry delay each time the transaction begins, so most code written assuming the older behavior can be upgraded to the newer behavior without requiring any modification, and the caller is not required to implement special logic in retry loops to only conditionally set this option.
    MaxRetryDelay(i32),
    /// value in bytes
    ///
    /// Set the transaction size limit in bytes. The size is calculated by combining the sizes of all keys and values written or mutated, all key ranges cleared, and all read and write conflict ranges. (In other words, it includes the total size of all data included in the request to the cluster to commit the transaction.) Large transactions can cause performance problems on FoundationDB clusters, so setting this limit to a smaller value than the default can help prevent the client from accidentally degrading the cluster's performance. This value must be at least 32 and cannot be set to higher than 10,000,000, the default transaction size limit.
    SizeLimit(i32),
    /// Automatically assign a random 16 byte idempotency id for this transaction. Prevents commits from failing with ``commit_unknown_result``. WARNING: If you are also using the multiversion client or transaction timeouts, if either cluster_version_changed or transaction_timed_out was thrown during a commit, then that commit may have already succeeded or may succeed in the future. This feature is in development and not ready for general use.
    AutomaticIdempotency,
    /// Snapshot read operations will see the results of writes done in the same transaction. This is the default behavior.
    SnapshotRywEnable,
    /// Snapshot read operations will not see the results of writes done in the same transaction. This was the default behavior prior to API version 300.
    SnapshotRywDisable,
    /// The transaction can read and write to locked databases, and is responsible for checking that it took the lock.
    LockAware,
    /// By default, operations that are performed on a transaction while it is being committed will not only fail themselves, but they will attempt to fail other in-flight operations (such as the commit) as well. This behavior is intended to help developers discover situations where operations could be unintentionally executed after the transaction has been reset. Setting this option removes that protection, causing only the offending operation to fail.
    UsedDuringCommitProtectionDisable,
    /// The transaction can read from locked databases.
    ReadLockAware,
    /// This option should only be used by tools which change the database configuration.
    UseProvisionalProxies,
    /// The transaction can retrieve keys that are conflicting with other transactions.
    ReportConflictingKeys,
    /// By default, the special key space will only allow users to read from exactly one module (a subspace in the special key space). Use this option to allow reading from zero or more modules. Users who set this option should be prepared for new modules, which may have different behaviors than the modules they're currently reading. For example, a new module might block or return an error.
    SpecialKeySpaceRelaxed,
    /// By default, users are not allowed to write to special keys. Enable this option will implicitly enable all options required to achieve the configuration change.
    SpecialKeySpaceEnableWrites,
    /// String identifier used to associated this transaction with a throttling group. Must not exceed 16 characters.
    ///
    /// Adds a tag to the transaction that can be used to apply manual targeted throttling. At most 5 tags can be set on a transaction.
    Tag(String),
    /// String identifier used to associated this transaction with a throttling group. Must not exceed 16 characters.
    ///
    /// Adds a tag to the transaction that can be used to apply manual or automatic targeted throttling. At most 5 tags can be set on a transaction.
    AutoThrottleTag(String),
    /// A byte string of length 16 used to associate the span of this transaction with a parent
    ///
    /// Adds a parent to the Span of this transaction. Used for transaction tracing. A span can be identified with any 16 bytes
    SpanParent(Vec<u8>),
    /// Asks storage servers for how many bytes a clear key range contains. Otherwise uses the location cache to roughly estimate this.
    ExpensiveClearCostEstimationEnable,
    /// Allows ``get`` operations to read from sections of keyspace that have become unreadable because of versionstamp operations. These reads will view versionstamp operations as if they were set operations that did not fill in the versionstamp.
    BypassUnreadable,
    /// Allows this transaction to use cached GRV from the database context. Defaults to off. Upon first usage, starts a background updater to periodically update the cache to avoid stale read versions. The disable_client_bypass option must also be set.
    UseGrvCache,
    /// A JSON Web Token authorized to access data belonging to one or more tenants, indicated by 'tenants' claim of the token's payload.
    ///
    /// Attach given authorization token to the transaction such that subsequent tenant-aware requests are authorized
    AuthorizationToken(String),
}
impl TransactionOption {
    pub fn code(&self) -> fdb_sys::FDBTransactionOption {
        match *self {
            TransactionOption::CausalWriteRisky => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_CAUSAL_WRITE_RISKY,
            TransactionOption::CausalReadRisky => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_CAUSAL_READ_RISKY,
            TransactionOption::CausalReadDisable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_CAUSAL_READ_DISABLE,
            TransactionOption::IncludePortInAddress => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_INCLUDE_PORT_IN_ADDRESS,
            TransactionOption::NextWriteNoWriteConflictRange => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_NEXT_WRITE_NO_WRITE_CONFLICT_RANGE,
            TransactionOption::ReadYourWritesDisable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_YOUR_WRITES_DISABLE,
            TransactionOption::ReadAheadDisable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_AHEAD_DISABLE,
            TransactionOption::ReadServerSideCacheEnable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_SERVER_SIDE_CACHE_ENABLE,
            TransactionOption::ReadServerSideCacheDisable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_SERVER_SIDE_CACHE_DISABLE,
            TransactionOption::ReadPriorityNormal => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_PRIORITY_NORMAL,
            TransactionOption::ReadPriorityLow => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_PRIORITY_LOW,
            TransactionOption::ReadPriorityHigh => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_PRIORITY_HIGH,
            TransactionOption::DurabilityDatacenter => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_DURABILITY_DATACENTER,
            TransactionOption::DurabilityRisky => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_DURABILITY_RISKY,
            TransactionOption::DurabilityDevNullIsWebScale => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_DURABILITY_DEV_NULL_IS_WEB_SCALE,
            TransactionOption::PrioritySystemImmediate => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_PRIORITY_SYSTEM_IMMEDIATE,
            TransactionOption::PriorityBatch => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_PRIORITY_BATCH,
            TransactionOption::InitializeNewDatabase => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_INITIALIZE_NEW_DATABASE,
            TransactionOption::AccessSystemKeys => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_ACCESS_SYSTEM_KEYS,
            TransactionOption::ReadSystemKeys => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_SYSTEM_KEYS,
            TransactionOption::RawAccess => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_RAW_ACCESS,
            TransactionOption::BypassStorageQuota => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_BYPASS_STORAGE_QUOTA,
            TransactionOption::DebugRetryLogging(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_DEBUG_RETRY_LOGGING,
            TransactionOption::TransactionLoggingEnable(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_TRANSACTION_LOGGING_ENABLE,
            TransactionOption::DebugTransactionIdentifier(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_DEBUG_TRANSACTION_IDENTIFIER,
            TransactionOption::LogTransaction => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_LOG_TRANSACTION,
            TransactionOption::TransactionLoggingMaxFieldLength(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_TRANSACTION_LOGGING_MAX_FIELD_LENGTH,
            TransactionOption::ServerRequestTracing => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SERVER_REQUEST_TRACING,
            TransactionOption::Timeout(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_TIMEOUT,
            TransactionOption::RetryLimit(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_RETRY_LIMIT,
            TransactionOption::MaxRetryDelay(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_MAX_RETRY_DELAY,
            TransactionOption::SizeLimit(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SIZE_LIMIT,
            TransactionOption::AutomaticIdempotency => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_AUTOMATIC_IDEMPOTENCY,
            TransactionOption::SnapshotRywEnable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SNAPSHOT_RYW_ENABLE,
            TransactionOption::SnapshotRywDisable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SNAPSHOT_RYW_DISABLE,
            TransactionOption::LockAware => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_LOCK_AWARE,
            TransactionOption::UsedDuringCommitProtectionDisable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_USED_DURING_COMMIT_PROTECTION_DISABLE,
            TransactionOption::ReadLockAware => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_READ_LOCK_AWARE,
            TransactionOption::UseProvisionalProxies => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_USE_PROVISIONAL_PROXIES,
            TransactionOption::ReportConflictingKeys => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_REPORT_CONFLICTING_KEYS,
            TransactionOption::SpecialKeySpaceRelaxed => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SPECIAL_KEY_SPACE_RELAXED,
            TransactionOption::SpecialKeySpaceEnableWrites => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SPECIAL_KEY_SPACE_ENABLE_WRITES,
            TransactionOption::Tag(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_TAG,
            TransactionOption::AutoThrottleTag(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_AUTO_THROTTLE_TAG,
            TransactionOption::SpanParent(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_SPAN_PARENT,
            TransactionOption::ExpensiveClearCostEstimationEnable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_EXPENSIVE_CLEAR_COST_ESTIMATION_ENABLE,
            TransactionOption::BypassUnreadable => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_BYPASS_UNREADABLE,
            TransactionOption::UseGrvCache => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_USE_GRV_CACHE,
            TransactionOption::AuthorizationToken(..) => fdb_sys::FDBTransactionOption_FDB_TR_OPTION_AUTHORIZATION_TOKEN,
        }
    }
    pub unsafe fn apply(&self, target: *mut fdb_sys::FDBTransaction) -> FdbResult<()> {
        let code = self.code();
        let err = match *self {
            TransactionOption::CausalWriteRisky => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::CausalReadRisky => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::CausalReadDisable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::IncludePortInAddress => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::NextWriteNoWriteConflictRange => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadYourWritesDisable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadAheadDisable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadServerSideCacheEnable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadServerSideCacheDisable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadPriorityNormal => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadPriorityLow => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadPriorityHigh => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::DurabilityDatacenter => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::DurabilityRisky => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::DurabilityDevNullIsWebScale => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::PrioritySystemImmediate => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::PriorityBatch => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::InitializeNewDatabase => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::AccessSystemKeys => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadSystemKeys => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::RawAccess => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::BypassStorageQuota => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::DebugRetryLogging(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            TransactionOption::TransactionLoggingEnable(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            TransactionOption::DebugTransactionIdentifier(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            TransactionOption::LogTransaction => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::TransactionLoggingMaxFieldLength(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_transaction_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            TransactionOption::ServerRequestTracing => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::Timeout(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_transaction_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            TransactionOption::RetryLimit(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_transaction_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            TransactionOption::MaxRetryDelay(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_transaction_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            TransactionOption::SizeLimit(v) => {
                let data: [u8;8] = std::mem::transmute(v as i64);
                fdb_sys::fdb_transaction_set_option(target, code, data.as_ptr() as *const u8, 8)
            }
            TransactionOption::AutomaticIdempotency => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::SnapshotRywEnable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::SnapshotRywDisable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::LockAware => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::UsedDuringCommitProtectionDisable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReadLockAware => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::UseProvisionalProxies => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::ReportConflictingKeys => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::SpecialKeySpaceRelaxed => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::SpecialKeySpaceEnableWrites => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::Tag(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            TransactionOption::AutoThrottleTag(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            TransactionOption::SpanParent(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
            TransactionOption::ExpensiveClearCostEstimationEnable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::BypassUnreadable => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::UseGrvCache => fdb_sys::fdb_transaction_set_option(target, code, std::ptr::null(), 0),
            TransactionOption::AuthorizationToken(ref v) => {
                fdb_sys::fdb_transaction_set_option(target, code, v.as_ptr() as *const u8, i32::try_from(v.len()).expect("len to fit in i32"))

            }
        };
        if err != 0 { Err(FdbError::from_code(err)) } else { Ok(()) }
    }
}
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum StreamingMode {
    /// Client intends to consume the entire range and would like it all transferred as early as possible.
    WantAll,
    /// The default. The client doesn't know how much of the range it is likely to used and wants different performance concerns to be balanced. Only a small portion of data is transferred to the client initially (in order to minimize costs if the client doesn't read the entire range), and as the caller iterates over more items in the range larger batches will be transferred in order to minimize latency. After enough iterations, the iterator mode will eventually reach the same byte limit as ``WANT_ALL``
    Iterator,
    /// Infrequently used. The client has passed a specific row limit and wants that many rows delivered in a single batch. Because of iterator operation in client drivers make request batches transparent to the user, consider ``WANT_ALL`` StreamingMode instead. A row limit must be specified if this mode is used.
    Exact,
    /// Infrequently used. Transfer data in batches small enough to not be much more expensive than reading individual rows, to minimize cost if iteration stops early.
    Small,
    /// Infrequently used. Transfer data in batches sized in between small and large.
    Medium,
    /// Infrequently used. Transfer data in batches large enough to be, in a high-concurrency environment, nearly as efficient as possible. If the client stops iteration early, some disk and network bandwidth may be wasted. The batch size may still be too small to allow a single client to get high throughput from the database, so if that is what you need consider the SERIAL StreamingMode.
    Large,
    /// Transfer data in batches large enough that an individual client can get reasonable read bandwidth from the database. If the client stops iteration early, considerable disk and network bandwidth may be wasted.
    Serial,
}
impl StreamingMode {
    pub fn code(&self) -> fdb_sys::FDBStreamingMode {
        match *self {
            StreamingMode::WantAll => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_WANT_ALL,
            StreamingMode::Iterator => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_ITERATOR,
            StreamingMode::Exact => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_EXACT,
            StreamingMode::Small => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_SMALL,
            StreamingMode::Medium => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_MEDIUM,
            StreamingMode::Large => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_LARGE,
            StreamingMode::Serial => fdb_sys::FDBStreamingMode_FDB_STREAMING_MODE_SERIAL,
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum MutationType {
    /// addend
    ///
    /// Performs an addition of little-endian integers. If the existing value in the database is not present or shorter than ``param``, it is first extended to the length of ``param`` with zero bytes.  If ``param`` is shorter than the existing value in the database, the existing value is truncated to match the length of ``param``. The integers to be added must be stored in a little-endian representation.  They can be signed in two's complement representation or unsigned. You can add to an integer at a known offset in the value by prepending the appropriate number of zero bytes to ``param`` and padding with zero bytes to match the length of the value. However, this offset technique requires that you know the addition will not cause the integer field within the value to overflow.
    Add,
    /// value with which to perform bitwise and
    ///
    /// Deprecated
    And,
    /// value with which to perform bitwise and
    ///
    /// Performs a bitwise ``and`` operation.  If the existing value in the database is not present, then ``param`` is stored in the database. If the existing value in the database is shorter than ``param``, it is first extended to the length of ``param`` with zero bytes.  If ``param`` is shorter than the existing value in the database, the existing value is truncated to match the length of ``param``.
    BitAnd,
    /// value with which to perform bitwise or
    ///
    /// Deprecated
    Or,
    /// value with which to perform bitwise or
    ///
    /// Performs a bitwise ``or`` operation.  If the existing value in the database is not present or shorter than ``param``, it is first extended to the length of ``param`` with zero bytes.  If ``param`` is shorter than the existing value in the database, the existing value is truncated to match the length of ``param``.
    BitOr,
    /// value with which to perform bitwise xor
    ///
    /// Deprecated
    Xor,
    /// value with which to perform bitwise xor
    ///
    /// Performs a bitwise ``xor`` operation.  If the existing value in the database is not present or shorter than ``param``, it is first extended to the length of ``param`` with zero bytes.  If ``param`` is shorter than the existing value in the database, the existing value is truncated to match the length of ``param``.
    BitXor,
    /// value to append to the database value
    ///
    /// Appends ``param`` to the end of the existing value already in the database at the given key (or creates the key and sets the value to ``param`` if the key is empty). This will only append the value if the final concatenated value size is less than or equal to the maximum value size (i.e., if it fits). WARNING: No error is surfaced back to the user if the final value is too large because the mutation will not be applied until after the transaction has been committed. Therefore, it is only safe to use this mutation type if one can guarantee that one will keep the total value size under the maximum size.
    AppendIfFits,
    /// value to check against database value
    ///
    /// Performs a little-endian comparison of byte strings. If the existing value in the database is not present or shorter than ``param``, it is first extended to the length of ``param`` with zero bytes.  If ``param`` is shorter than the existing value in the database, the existing value is truncated to match the length of ``param``. The larger of the two values is then stored in the database.
    Max,
    /// value to check against database value
    ///
    /// Performs a little-endian comparison of byte strings. If the existing value in the database is not present, then ``param`` is stored in the database. If the existing value in the database is shorter than ``param``, it is first extended to the length of ``param`` with zero bytes.  If ``param`` is shorter than the existing value in the database, the existing value is truncated to match the length of ``param``. The smaller of the two values is then stored in the database.
    Min,
    /// value to which to set the transformed key
    ///
    /// Transforms ``key`` using a versionstamp for the transaction. Sets the transformed key in the database to ``param``. The key is transformed by removing the final four bytes from the key and reading those as a little-Endian 32-bit integer to get a position ``pos``. The 10 bytes of the key from ``pos`` to ``pos + 10`` are replaced with the versionstamp of the transaction used. The first byte of the key is position 0. A versionstamp is a 10 byte, unique, monotonically (but not sequentially) increasing value for each committed transaction. The first 8 bytes are the committed version of the database (serialized in big-Endian order). The last 2 bytes are monotonic in the serialization order for transactions. WARNING: At this time, versionstamps are compatible with the Tuple layer only in the Java, Python, and Go bindings. Also, note that prior to API version 520, the offset was computed from only the final two bytes rather than the final four bytes.
    SetVersionstampedKey,
    /// value to versionstamp and set
    ///
    /// Transforms ``param`` using a versionstamp for the transaction. Sets the ``key`` given to the transformed ``param``. The parameter is transformed by removing the final four bytes from ``param`` and reading those as a little-Endian 32-bit integer to get a position ``pos``. The 10 bytes of the parameter from ``pos`` to ``pos + 10`` are replaced with the versionstamp of the transaction used. The first byte of the parameter is position 0. A versionstamp is a 10 byte, unique, monotonically (but not sequentially) increasing value for each committed transaction. The first 8 bytes are the committed version of the database (serialized in big-Endian order). The last 2 bytes are monotonic in the serialization order for transactions. WARNING: At this time, versionstamps are compatible with the Tuple layer only in the Java, Python, and Go bindings. Also, note that prior to API version 520, the versionstamp was always placed at the beginning of the parameter rather than computing an offset.
    SetVersionstampedValue,
    /// value to check against database value
    ///
    /// Performs lexicographic comparison of byte strings. If the existing value in the database is not present, then ``param`` is stored. Otherwise the smaller of the two values is then stored in the database.
    ByteMin,
    /// value to check against database value
    ///
    /// Performs lexicographic comparison of byte strings. If the existing value in the database is not present, then ``param`` is stored. Otherwise the larger of the two values is then stored in the database.
    ByteMax,
    /// Value to compare with
    ///
    /// Performs an atomic ``compare and clear`` operation. If the existing value in the database is equal to the given value, then given key is cleared.
    CompareAndClear,
}
impl MutationType {
    pub fn code(&self) -> fdb_sys::FDBMutationType {
        match *self {
            MutationType::Add => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_ADD,
            MutationType::And => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_AND,
            MutationType::BitAnd => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_BIT_AND,
            MutationType::Or => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_OR,
            MutationType::BitOr => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_BIT_OR,
            MutationType::Xor => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_XOR,
            MutationType::BitXor => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_BIT_XOR,
            MutationType::AppendIfFits => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_APPEND_IF_FITS,
            MutationType::Max => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_MAX,
            MutationType::Min => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_MIN,
            MutationType::SetVersionstampedKey => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_SET_VERSIONSTAMPED_KEY,
            MutationType::SetVersionstampedValue => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_SET_VERSIONSTAMPED_VALUE,
            MutationType::ByteMin => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_BYTE_MIN,
            MutationType::ByteMax => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_BYTE_MAX,
            MutationType::CompareAndClear => fdb_sys::FDBMutationType_FDB_MUTATION_TYPE_COMPARE_AND_CLEAR,
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ConflictRangeType {
    /// Used to add a read conflict range
    Read,
    /// Used to add a write conflict range
    Write,
}
impl ConflictRangeType {
    pub fn code(&self) -> fdb_sys::FDBConflictRangeType {
        match *self {
            ConflictRangeType::Read => fdb_sys::FDBConflictRangeType_FDB_CONFLICT_RANGE_TYPE_READ,
            ConflictRangeType::Write => fdb_sys::FDBConflictRangeType_FDB_CONFLICT_RANGE_TYPE_WRITE,
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ErrorPredicate {
    /// Returns ``true`` if the error indicates the operations in the transactions should be retried because of transient error.
    Retryable,
    /// Returns ``true`` if the error indicates the transaction may have succeeded, though not in a way the system can verify.
    MaybeCommitted,
    /// Returns ``true`` if the error indicates the transaction has not committed, though in a way that can be retried.
    RetryableNotCommitted,
}
impl ErrorPredicate {
    pub fn code(&self) -> fdb_sys::FDBErrorPredicate {
        match *self {
            ErrorPredicate::Retryable => fdb_sys::FDBErrorPredicate_FDB_ERROR_PREDICATE_RETRYABLE,
            ErrorPredicate::MaybeCommitted => fdb_sys::FDBErrorPredicate_FDB_ERROR_PREDICATE_MAYBE_COMMITTED,
            ErrorPredicate::RetryableNotCommitted => fdb_sys::FDBErrorPredicate_FDB_ERROR_PREDICATE_RETRYABLE_NOT_COMMITTED,
        }
    }
}
