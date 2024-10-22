#[derive(Debug)]
pub enum TransactionErrorDecodeError {
    EmptyError,
    UnknownErrorCode(u8),
    InvalidInstructionError,
    BufferTooShort,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionError {
    AccountInUse,
    AccountLoadedTwice,
    AccountNotFound,
    ProgramAccountNotFound,
    InsufficientFundsForFee,
    InvalidAccountForFee,
    AlreadyProcessed,
    BlockhashNotFound,
    InstructionError(u32, InstructionError),
    CallChainTooDeep,
    MissingSignatureForFee,
    InvalidAccountIndex,
    SignatureFailure,
    InvalidProgramForExecution,
    SanitizeFailure,
    ClusterMaintenance,
    AccountBorrowOutstanding,
    WouldExceedMaxBlockCostLimit,
    UnsupportedVersion,
    InvalidWritableAccount,
    WouldExceedMaxAccountCostLimit,
    WouldExceedAccountDataBlockLimit,
    TooManyAccountLocks,
    AddressLookupTableNotFound,
    InvalidAddressLookupTableOwner,
    InvalidAddressLookupTableData,
    InvalidAddressLookupTableIndex,
    InvalidRentPayingAccount,
    WouldExceedMaxVoteCostLimit,
    WouldExceedAccountDataTotalLimit,
    DuplicateInstruction(u8),
    InsufficientFundsForRent { account_index: u32 },
    MaxLoadedAccountsDataSizeExceeded,
    InvalidLoadedAccountsDataSizeLimit,
    ResanitizationNeeded,
    ProgramExecutionTemporarilyRestricted { account_index: u32 },
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
    Custom(u32),
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
    fn from_u32(value: u32) -> Self {
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
            _ => InstructionError::Custom(value as u32),
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
                let instruction_index = Self::extract_index(error_data)?;
                let error_number = u32::from_le_bytes(error_data[error_data.len() - 4..].try_into().unwrap());
                Ok(TransactionError::InstructionError(instruction_index, InstructionError::from_u32(error_number)))
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
            31 => Ok(TransactionError::InsufficientFundsForRent {
                account_index: Self::extract_index(error_data)?,
            }),
            32 => Ok(TransactionError::MaxLoadedAccountsDataSizeExceeded),
            33 => Ok(TransactionError::InvalidLoadedAccountsDataSizeLimit),
            34 => Ok(TransactionError::ResanitizationNeeded),
            35 => Ok(TransactionError::ProgramExecutionTemporarilyRestricted {
                account_index: Self::extract_index(error_data)?,
            }),
            36 => Ok(TransactionError::UnbalancedTransaction),
            _ => Err(TransactionErrorDecodeError::UnknownErrorCode(error_code)),
        }
    }

    fn extract_index(error_data: &[u8]) -> Result<u32, TransactionErrorDecodeError> {
        if error_data.len() < 5 {
            return Err(TransactionErrorDecodeError::BufferTooShort);
        }
        Ok(u32::from_be_bytes(error_data[1..5].try_into().unwrap()))
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
