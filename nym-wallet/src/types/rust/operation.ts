export type Operation =
  | "Upload"
  | "Init"
  | "Migrate"
  | "ChangeAdmin"
  | "Send"
  | "BondMixnode"
  | "BondMixnodeOnBehalf"
  | "UnbondMixnode"
  | "UnbondMixnodeOnBehalf"
  | "DelegateToMixnode"
  | "DelegateToMixnodeOnBehalf"
  | "UndelegateFromMixnode"
  | "UndelegateFromMixnodeOnBehalf"
  | "BondGateway"
  | "BondGatewayOnBehalf"
  | "UnbondGateway"
  | "UnbondGatewayOnBehalf"
  | "UpdateContractSettings"
  | "BeginMixnodeRewarding"
  | "FinishMixnodeRewarding"
  | "TrackUnbondGateway"
  | "TrackUnbondMixnode"
  | "WithdrawVestedCoins"
  | "TrackUndelegation"
  | "CreatePeriodicVestingAccount";