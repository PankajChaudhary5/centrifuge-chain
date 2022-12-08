window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is used to limit the maximal weight of a single extrinsic."],["CENTI_CFG",""],["CFG",""],["DAYS",""],["DEFAULT_FEE_VALUE","Value for a not specified fee key."],["HOURS",""],["MAXIMUM_BLOCK_WEIGHT","We allow for 0.5 seconds of compute with a 6 second average block time."],["MAX_MULTISIG_CALL_SIZE","Maximum size of a multisig call allowed by the base filters - 50Kib TODO: Remove once we have upgraded to Substrate/Polkadot v0.9.31"],["MAX_TOKEN_NAME_LENGTH_BYTES","The max length allowed for a tranche token name"],["MAX_TOKEN_SYMBOL_LENGTH_BYTES","The max length allowed for a tranche token symbol"],["MICRO_CFG",""],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MILLISECS_PER_DAY","Milliseconds per day"],["MILLI_CFG",""],["MINUTES",""],["MIN_VESTING","Minimum vesting amount, in CFG/AIR"],["NFTS_PREFIX","Represents the protobuf encoding - “NFTS”. All Centrifuge documents are formatted in this way. These are pre/appended to the registry id before being set as a [RegistryInfo] field in [create_registry]."],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by Operational  extrinsics."],["SECONDS_PER_DAY",""],["SECONDS_PER_HOUR",""],["SECONDS_PER_MINUTE",""],["SECONDS_PER_YEAR",""],["SLOT_DURATION",""],["TREASURY_FEE_RATIO","% of fee addressed to the Treasury. The reminder % will be for the block author."],["VERSION","Runtime version."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["Call",""],["CurrencyId",""],["Event",""],["OriginCaller",""],["ProxyType","The type used to represent the kinds of proxying allowed."],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["deposit",""],["native_version","Native version."]],"mod":[["api",""],["apis","Runtime apis useful in the Centrifuge ecosystem"],["asset_registry","AssetRegistry’s AssetProcessor"],["constants","A set of constant values used in altair runtime"],["currency_decimals",""],["fees",""],["xcm",""],["xcm_fees",""]],"struct":[["AnnouncementDepositBase",""],["AnnouncementDepositFactor",""],["AttributeDepositBase",""],["BaseCallFilter","Base Call Filter"],["BasicDeposit",""],["Burn",""],["CandidacyBond",""],["ChallengeTime",""],["CollectionDeposit",""],["CommitAnchorFeeKey",""],["CooloffPeriod",""],["CouncilMaxMembers",""],["CouncilMaxProposals",""],["CouncilMotionDuration",""],["CrowdloanClaimPalletId",""],["CrowdloanRewardPalletId",""],["CurrencyAdapter","Implements the transaction payment for a pallet implementing the `Currency` trait (eg. the pallet_balances) using an unbalance handler (implementing `OnUnbalanced`)."],["DefaultFeeValue",""],["DefaultMaxNAVAge",""],["DefaultMinEpochTime",""],["DepositBase",""],["DepositFactor",""],["DepositPerByte",""],["DesiredMembers",""],["DesiredRunnersUp",""],["Editors",""],["ElectionsPhragmenModuleId",""],["EnactmentPeriod",""],["EthAddress","A generic representation of a local address. A resource id points to this. It may be a registry id (20 bytes) or a fungible asset type (in the future). Constrained to 32 bytes just as an upper bound to store efficiently."],["ExistentialDeposit",""],["ExistentialDeposits",""],["FastTrackVotingPeriod",""],["FieldDeposit",""],["GenesisConfig",""],["InstantAllowed",""],["IsTrancheInvestor","Checks whether the given `who` has the role of a `TrancehInvestor` for the given pool."],["ItemDeposit",""],["ItemId","A representation of ItemId for Uniques."],["LaunchPeriod",""],["Limit",""],["LoansPalletId",""],["MaxActiveLoansPerPool",""],["MaxAdditionalFields",""],["MaxApprovals",""],["MaxAuthorities",""],["MaxCandidates",""],["MaxInvulnerables",""],["MaxLocks",""],["MaxNAVAgeUpperBound",""],["MaxOutstandingCollects",""],["MaxPending",""],["MaxProofLength",""],["MaxProposals",""],["MaxProxies",""],["MaxRegistrars",""],["MaxReserves",""],["MaxRolesPerPool",""],["MaxScheduledPerBlock",""],["MaxSignatories",""],["MaxSizeMetadata",""],["MaxSubAccounts",""],["MaxTranches",""],["MaxVoters",""],["MaxVotes",""],["MaxWriteOffGroups",""],["MaximumBlockWeight",""],["MaximumSchedulerWeight",""],["MetadataDepositBase",""],["MigrationMaxAccounts",""],["MigrationMaxProxies",""],["MigrationMaxVestings",""],["MinCandidates",""],["MinDelay",""],["MinEpochTimeLowerBound",""],["MinEpochTimeUpperBound",""],["MinUpdateDelay",""],["MinVestedTransfer",""],["MinimumDeposit",""],["MinimumPeriod",""],["NativeToken",""],["NftSalesPalletId",""],["NoPreimagePostponement",""],["Offset",""],["OperationalFeeMultiplier","This value increases the priority of `Operational` transactions by adding a “virtual tip” that’s equal to the `OperationalFeeMultiplier * final_fee`."],["Origin","The runtime origin type representing the origin of a call."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Period",""],["PoolCurrency",""],["PoolDeposit",""],["PoolPalletId",""],["PoolPalletIndex","The index with which this pallet is instantiated in this runtime."],["PotId",""],["PreCommitDepositFeeKey",""],["PreimageBaseDeposit",""],["PreimageByteDeposit",""],["PreimageMaxSize",""],["ProposalBond",""],["ProposalBondMaximum",""],["ProposalBondMinimum",""],["ProxyDepositBase",""],["ProxyDepositFactor",""],["RegistryId","A representation of registryID."],["ReservedDmpWeight",""],["ReservedXcmpWeight",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeBlockLength",""],["RuntimeBlockWeights",""],["SS58Prefix",""],["SessionKeys",""],["SessionLength",""],["SpendPeriod",""],["SubAccountDeposit",""],["TargetedFeeAdjustment","A struct to update the weight multiplier per block. It implements `Convert<Multiplier, Multiplier>`, meaning that it can convert the previous multiplier to the next one. This should be called on `on_finalize` of a block, prior to potentially cleaning the weight data from the system pallet."],["TermDuration",""],["TokenId",""],["TrancheWeight","A representation of a tranche weight, used to weight importance of a tranche"],["TransactionByteFee","TransactionByteFee is set to 0.01 MicroAIR"],["TreasuryAccount",""],["TreasuryPalletId",""],["UncleGenerations",""],["UpdateGuard",""],["Version",""],["VotingBond",""],["VotingBondBase",""],["VotingPeriod",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them, but you never know…"],["Address","The address format for describing accounts."],["AllOfCouncil","All council members must vote yes to create this origin."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Anchor",""],["Aura",""],["AuraConfig",""],["AuraExt",""],["AuraExtConfig",""],["AuraId","Aura consensus authority."],["Authorship",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["BlockNumber","An index to a block."],["Bytes",""],["Bytes32",""],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CollatorAllowlist",""],["CollatorAllowlistConfig",""],["CollatorSelection",""],["CollatorSelectionConfig",""],["CollectionId","A representation of CollectionId for Uniques"],["Council",""],["CouncilCollective","The council"],["CouncilConfig",""],["CrowdloanClaim",""],["CrowdloanReward",""],["CumulusXcm",""],["Democracy",""],["DemocracyConfig",""],["DmpQueue",""],["Elections",""],["ElectionsConfig",""],["EnsureRootOr",""],["Executive","Executive: handles dispatch to the various modules."],["Fees",""],["FeesConfig",""],["FixedArray",""],["HalfOfCouncil","1/2 of all council members must vote yes to create this origin."],["Hash","A hash of some data used by the chain."],["Hashing","The hashing algorithm used by the chain"],["Header","Block header type as expected by this runtime."],["IBalance","IBalance is the signed version of the Balance for orml tokens"],["Identity",""],["Index","Index of a transaction in the chain."],["InterestAccrual",""],["InterestAccrualConfig",""],["Investments",""],["Loans",""],["Migration",""],["Moment","Moment type"],["Multiplier","Fee multiplier."],["Multisig",""],["NftSales",""],["OrderId","OrderId type we use to identify order per epoch."],["OrmlAssetRegistry",""],["OrmlAssetRegistryConfig",""],["OrmlTokens",""],["OrmlTokensConfig",""],["OrmlXcm",""],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainSystem",""],["ParachainSystemConfig",""],["Permissions",""],["PolkadotXcm",""],["PoolEpochId","EpochId type we use to identify epochs in our revolving pools"],["PoolId","PoolId type we use."],["PoolSystem",""],["Preimage",""],["Proxy",""],["RandomnessCollectiveFlip",""],["Salt",""],["Scheduler",""],["Session",""],["SessionConfig",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["System",""],["SystemConfig",""],["ThreeFourthOfCouncil","3/4 of all council members must vote yes to create this origin."],["Timestamp",""],["Tokens",""],["TrancheId","A representation of a tranche identifier"],["TransactionPayment",""],["Treasury",""],["TreasuryConfig",""],["TwoThirdOfCouncil","2/3 of all council members must vote yes to create this origin."],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Uniques",""],["Utility",""],["Vesting",""],["VestingConfig",""],["XTokens",""],["XcmpQueue",""]]};