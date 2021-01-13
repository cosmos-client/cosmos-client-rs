# CosmosBaseAbciV1beta1TxResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**height** | Option<**String**> |  | [optional]
**txhash** | Option<**String**> | The transaction hash. | [optional]
**codespace** | Option<**String**> |  | [optional]
**code** | Option<**i64**> | Response code. | [optional]
**data** | Option<**String**> | Result bytes, if any. | [optional]
**raw_log** | Option<**String**> | The output of the application's logger (raw string). May be non-deterministic. | [optional]
**logs** | Option<[**Vec<crate::models::InlineResponse20071TxResponseLogs>**](inline_response_200_71_tx_response_logs.md)> | The output of the application's logger (typed). May be non-deterministic. | [optional]
**info** | Option<**String**> | Additional information. May be non-deterministic. | [optional]
**gas_wanted** | Option<**String**> | Amount of gas requested for transaction. | [optional]
**gas_used** | Option<**String**> | Amount of gas consumed by transaction. | [optional]
**tx** | Option<[**crate::models::InlineResponse20071TxResponseTx**](inline_response_200_71_tx_response_tx.md)> |  | [optional]
**timestamp** | Option<**String**> | Time of the previous block. For heights > 1, it's the weighted median of the timestamps of the valid votes in the block.LastCommit. For height == 1, it's genesis time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


