#[derive(Debug)]
pub enum TransactionErrorDecodeError {
    EmptyError,
    UnknownErrorCode(u8),
    InvalidInstructionError,
    BufferTooShort,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionError {
    /// An account is already being processed in another transaction in a way
    /// that does not support parallelism
    AccountInUse,

    /// A `Pubkey` appears twice in the transaction's `account_keys`.  Instructions can reference
    /// `Pubkey`s more than once but the message must contain a list with no duplicate keys
    AccountLoadedTwice,

    /// Attempt to debit an account but found no record of a prior credit.
    AccountNotFound,

    /// Attempt to load a program that does not exist
    ProgramAccountNotFound,

    /// The from `Pubkey` does not have sufficient balance to pay the fee to schedule the transaction
    InsufficientFundsForFee,

    /// This account may not be used to pay transaction fees
    InvalidAccountForFee,

    /// The bank has seen this transaction before. This can occur under normal operation
    /// when a UDP packet is duplicated, as a user error from a client not updating
    /// its `recent_blockhash`, or as a double-spend attack.
    AlreadyProcessed,

    /// The bank has not seen the given `recent_blockhash` or the transaction is too old and
    /// the `recent_blockhash` has been discarded.
    BlockhashNotFound,

    /// An error occurred while processing an instruction. The first element of the tuple
    /// indicates the instruction index in which the error occurred.
    InstructionError(u8, InstructionError),

    /// Loader call chain is too deep
    CallChainTooDeep,

    /// Transaction requires a fee but has no signature present
    MissingSignatureForFee,

    /// Transaction contains an invalid account reference
    InvalidAccountIndex,

    /// Transaction did not pass signature verification
    SignatureFailure,

    /// This program may not be used for executing instructions
    InvalidProgramForExecution,

    /// Transaction failed to sanitize accounts offsets correctly
    /// implies that account locks are not taken for this TX, and should
    /// not be unlocked.
    SanitizeFailure,

    /// Transactions are currently disabled due to cluster maintenance
    ClusterMaintenance,

    /// Transaction processing left an account with an outstanding borrowed reference
    AccountBorrowOutstanding,

    /// Transaction would exceed max Block Cost Limit
    WouldExceedMaxBlockCostLimit,

    /// Transaction version is unsupported
    UnsupportedVersion,

    /// Transaction loads a writable account that cannot be written
    InvalidWritableAccount,

    /// Transaction would exceed max account limit within the block
    WouldExceedMaxAccountCostLimit,

    /// Transaction would exceed account data limit within the block
    WouldExceedAccountDataBlockLimit,

    /// Transaction locked too many accounts
    TooManyAccountLocks,

    /// Address lookup table not found
    AddressLookupTableNotFound,

    /// Attempted to lookup addresses from an account owned by the wrong program
    InvalidAddressLookupTableOwner,

    /// Attempted to lookup addresses from an invalid account
    InvalidAddressLookupTableData,

    /// Address table lookup uses an invalid index
    InvalidAddressLookupTableIndex,

    /// Transaction leaves an account with a lower balance than rent-exempt minimum
    InvalidRentPayingAccount,

    /// Transaction would exceed max Vote Cost Limit
    WouldExceedMaxVoteCostLimit,

    /// Transaction would exceed total account data limit
    WouldExceedAccountDataTotalLimit,

    /// Transaction contains a duplicate instruction that is not allowed
    DuplicateInstruction(u8),

    /// Transaction results in an account with insufficient funds for rent
    InsufficientFundsForRent { account_index: u8 },

    /// Transaction exceeded max loaded accounts data size cap
    MaxLoadedAccountsDataSizeExceeded,

    /// LoadedAccountsDataSizeLimit set for transaction must be greater than 0.
    InvalidLoadedAccountsDataSizeLimit,

    /// Sanitized transaction differed before/after feature activiation. Needs to be resanitized.
    ResanitizationNeeded,

    /// Program execution is temporarily restricted on an account.
    ProgramExecutionTemporarilyRestricted { account_index: u8 },

    /// The total balance before the transaction does not equal the total balance after the transaction
    UnbalancedTransaction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionError {
    GenericError,
    InvalidArgument,
    InvalidInstructionData,
    InvalidAccountData,
    AccountDataTooSmall,
    InsufficientFunds,
    IncorrectProgramId,
    MissingRequiredSignature,
    AccountAlreadyInitialized,
    UninitializedAccount,
    UnbalancedInstruction,
    ModifiedProgramId,
    ExternalAccountLamportSpend,
    ExternalAccountDataModified,
    ReadonlyLamportChange,
    ReadonlyDataModified,
    DuplicateAccountIndex,
    ExecutableModified,
    RentEpochModified,
    NotEnoughAccountKeys,
    AccountDataSizeChanged,
    AccountNotExecutable,
    AccountBorrowFailed,
    AccountBorrowOutstanding,
    DuplicateAccountOutOfSync,
    Custom(u8),
    InvalidError,
    ExecutableDataModified,
    ExecutableLamportChange,
    ExecutableAccountNotRentExempt,
    UnsupportedProgramId,
    CallDepth,
    MissingAccount,
    ReentrancyNotAllowed,
    MaxSeedLengthExceeded,
    InvalidSeeds,
    InvalidRealloc,
    ComputationalBudgetExceeded,
    PrivilegeEscalation,
    ProgramEnvironmentSetupFailure,
    ProgramFailedToComplete,
    ProgramFailedToCompile,
    Immutable,
    IncorrectAuthority,
    BorshIoError,
    AccountNotRentExempt,
    InvalidAccountOwner,
    ArithmeticOverflow,
    UnsupportedSysvar,
    IllegalOwner,
    MaxAccountsDataAllocationsExceeded,
    MaxAccountsExceeded,
    MaxInstructionTraceLengthExceeded,
    BuiltinProgramsMustConsumeComputeUnits,
}

impl InstructionError {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => InstructionError::GenericError,
            1 => InstructionError::InvalidArgument,
            2 => InstructionError::InvalidInstructionData,
            3 => InstructionError::InvalidAccountData,
            4 => InstructionError::AccountDataTooSmall,
            5 => InstructionError::InsufficientFunds,
            6 => InstructionError::IncorrectProgramId,
            7 => InstructionError::MissingRequiredSignature,
            8 => InstructionError::AccountAlreadyInitialized,
            9 => InstructionError::UninitializedAccount,
            10 => InstructionError::UnbalancedInstruction,
            11 => InstructionError::ModifiedProgramId,
            12 => InstructionError::ExternalAccountLamportSpend,
            13 => InstructionError::ExternalAccountDataModified,
            14 => InstructionError::ReadonlyLamportChange,
            15 => InstructionError::ReadonlyDataModified,
            16 => InstructionError::DuplicateAccountIndex,
            17 => InstructionError::ExecutableModified,
            18 => InstructionError::RentEpochModified,
            19 => InstructionError::NotEnoughAccountKeys,
            20 => InstructionError::AccountDataSizeChanged,
            21 => InstructionError::AccountNotExecutable,
            22 => InstructionError::AccountBorrowFailed,
            23 => InstructionError::AccountBorrowOutstanding,
            24 => InstructionError::DuplicateAccountOutOfSync,
            25 => InstructionError::InvalidError,
            26 => InstructionError::ExecutableDataModified,
            27 => InstructionError::ExecutableLamportChange,
            28 => InstructionError::ExecutableAccountNotRentExempt,
            29 => InstructionError::UnsupportedProgramId,
            30 => InstructionError::CallDepth,
            31 => InstructionError::MissingAccount,
            32 => InstructionError::ReentrancyNotAllowed,
            33 => InstructionError::MaxSeedLengthExceeded,
            34 => InstructionError::InvalidSeeds,
            35 => InstructionError::InvalidRealloc,
            36 => InstructionError::ComputationalBudgetExceeded,
            37 => InstructionError::PrivilegeEscalation,
            38 => InstructionError::ProgramEnvironmentSetupFailure,
            39 => InstructionError::ProgramFailedToComplete,
            40 => InstructionError::ProgramFailedToCompile,
            41 => InstructionError::Immutable,
            42 => InstructionError::IncorrectAuthority,
            43 => InstructionError::BorshIoError,
            44 => InstructionError::AccountNotRentExempt,
            45 => InstructionError::InvalidAccountOwner,
            46 => InstructionError::ArithmeticOverflow,
            47 => InstructionError::UnsupportedSysvar,
            48 => InstructionError::IllegalOwner,
            49 => InstructionError::MaxAccountsDataAllocationsExceeded,
            50 => InstructionError::MaxAccountsExceeded,
            51 => InstructionError::MaxInstructionTraceLengthExceeded,
            52 => InstructionError::BuiltinProgramsMustConsumeComputeUnits,
            _ => InstructionError::Custom(value),
        }
    }
}

pub struct TransactionErrorDecoder;

impl TransactionErrorDecoder {
    /// Decode a transaction error from Vec<u8>
    pub fn decode_error(error_data: &[u8]) -> Result<TransactionError, TransactionErrorDecodeError> {
        if error_data.is_empty() {
            return Err(TransactionErrorDecodeError::EmptyError);
        }

        let error_code = error_data[0];
        match error_code {
            0 => Ok(TransactionError::AccountInUse),
            1 => Ok(TransactionError::AccountLoadedTwice),
            2 => Ok(TransactionError::AccountNotFound),
            3 => Ok(TransactionError::ProgramAccountNotFound),
            4 => Ok(TransactionError::InsufficientFundsForFee),
            5 => Ok(TransactionError::InvalidAccountForFee),
            6 => Ok(TransactionError::AlreadyProcessed),
            7 => Ok(TransactionError::BlockhashNotFound),
            8 => {
                // InstructionError needs additional parsing
                if error_data.len() < 3 {
                    return Err(TransactionErrorDecodeError::InvalidInstructionError);
                }
                let instruction_index = error_data[1];
                let error_type = error_data[2];
                Ok(TransactionError::InstructionError(instruction_index, InstructionError::from_u8(error_type)))
            }
            9 => Ok(TransactionError::CallChainTooDeep),
            10 => Ok(TransactionError::MissingSignatureForFee),
            11 => Ok(TransactionError::InvalidAccountIndex),
            12 => Ok(TransactionError::SignatureFailure),
            13 => Ok(TransactionError::InvalidProgramForExecution),
            14 => Ok(TransactionError::SanitizeFailure),
            15 => Ok(TransactionError::ClusterMaintenance),
            16 => Ok(TransactionError::AccountBorrowOutstanding),
            17 => Ok(TransactionError::WouldExceedMaxBlockCostLimit),
            18 => Ok(TransactionError::UnsupportedVersion),
            19 => Ok(TransactionError::InvalidWritableAccount),
            20 => Ok(TransactionError::WouldExceedMaxAccountCostLimit),
            21 => Ok(TransactionError::WouldExceedAccountDataBlockLimit),
            22 => Ok(TransactionError::TooManyAccountLocks),
            23 => Ok(TransactionError::AddressLookupTableNotFound),
            24 => Ok(TransactionError::InvalidAddressLookupTableOwner),
            25 => Ok(TransactionError::InvalidAddressLookupTableData),
            26 => Ok(TransactionError::InvalidAddressLookupTableIndex),
            27 => Ok(TransactionError::InvalidRentPayingAccount),
            28 => Ok(TransactionError::WouldExceedMaxVoteCostLimit),
            29 => Ok(TransactionError::WouldExceedAccountDataTotalLimit),
            30 => {
                if error_data.len() < 2 {
                    return Err(TransactionErrorDecodeError::BufferTooShort);
                }
                Ok(TransactionError::DuplicateInstruction(error_data[1]))
            }
            31 => {
                if error_data.len() < 2 {
                    return Err(TransactionErrorDecodeError::BufferTooShort);
                }
                Ok(TransactionError::InsufficientFundsForRent { account_index: error_data[1] })
            }
            32 => Ok(TransactionError::MaxLoadedAccountsDataSizeExceeded),
            33 => Ok(TransactionError::InvalidLoadedAccountsDataSizeLimit),
            34 => Ok(TransactionError::ResanitizationNeeded),
            35 => {
                if error_data.len() < 2 {
                    return Err(TransactionErrorDecodeError::BufferTooShort);
                }
                Ok(TransactionError::ProgramExecutionTemporarilyRestricted { account_index: error_data[1] })
            }
            36 => Ok(TransactionError::UnbalancedTransaction),
            _ => Err(TransactionErrorDecodeError::UnknownErrorCode(error_code)),
        }
    }

    /// Format the error into a human-readable string
    pub fn format_error(error: &TransactionError) -> String {
        match error {
            TransactionError::InstructionError(idx, instruction_error) => {
                format!("Instruction error at index {}: {:?}", idx, instruction_error)
            }
            _ => format!("{:?}", error),
        }
    }
}
