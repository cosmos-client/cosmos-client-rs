# CosmosStakingV1beta1Validator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operator_address** | Option<**String**> |  | [optional]
**consensus_pubkey** | Option<[**crate::models::InlineResponseDefaultDetails**](inline_response_default_details.md)> |  | [optional]
**jailed** | Option<**bool**> |  | [optional]
**status** | Option<**String**> | BondStatus is the status of a validator.   - BOND_STATUS_UNSPECIFIED: UNSPECIFIED defines an invalid validator status.  - BOND_STATUS_UNBONDED: UNBONDED defines a validator that is not bonded.  - BOND_STATUS_UNBONDING: UNBONDING defines a validator that is unbonding.  - BOND_STATUS_BONDED: BONDED defines a validator that is bonded. | [optional][default to Status_UNSPECIFIED]
**tokens** | Option<**String**> |  | [optional]
**delegator_shares** | Option<**String**> |  | [optional]
**description** | Option<[**crate::models::InlineResponse20062Description**](inline_response_200_62_description.md)> |  | [optional]
**unbonding_height** | Option<**String**> |  | [optional]
**unbonding_time** | Option<**String**> |  | [optional]
**commission** | Option<[**crate::models::InlineResponse20062Commission**](inline_response_200_62_commission.md)> |  | [optional]
**min_self_delegation** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


