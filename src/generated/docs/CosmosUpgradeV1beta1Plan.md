# CosmosUpgradeV1beta1Plan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Sets the name for the upgrade. This name will be used by the upgraded version of the software to apply any special \"on-upgrade\" commands during the first BeginBlock method after the upgrade is applied. It is also used to detect whether a software version can handle a given upgrade. If no upgrade handler with this name has been set in the software, it will be assumed that the software is out-of-date when the upgrade Time or Height is reached and the software will exit. | [optional]
**time** | Option<**String**> | The time after which the upgrade must be performed. Leave set to its zero value to use a pre-defined Height instead. | [optional]
**height** | Option<**String**> | The height at which the upgrade must be performed. Only used if Time is not set. | [optional]
**info** | Option<**String**> |  | [optional]
**upgraded_client_state** | Option<[**crate::models::IbcEnabledChainsCanOptInToIncludingTheUpgradedClientStateInItsUpgradePlanThisWillMakeTheChainCommitToTheCorrectUpgradedSelfClientStateBeforeTheUpgradeOccursSoThatConnectingChainsCanVerifyThatTheNewUpgradedClientIsValidByVerifyingAProofOnThePreviousVersionOfTheChainThisWillAllowIbcConnectionsToPersistSmoothlyAcrossPlannedChainUpgrades**](IBC_enabled_chains_can_opt_in_to_including_the_upgraded_client_state_in_its_upgrade_plan_This_will_make_the_chain_commit_to_the_correct_upgraded__self__client_state_before_the_upgrade_occurs__so_that_connecting_chains_can_verify_that_the_new_upgraded_client_is_valid_by_verifying_a_proof_on_the_previous_version_of_the_chain__This_will_allow_IBC_connections_to_persist_smoothly_across_planned_chain_upgrades.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


