/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface CustomJson {
  id: string
  requiredAuths: Array<string>
  requiredPostingAuths: Array<string>
  json: string
}
export interface DynamicGlobalProperties {
  headBlockId: string
}
export function getDynamicGlobalProperties(): Promise<DynamicGlobalProperties>
export type JsClient = HiveClient
export class HiveClient {
  constructor(target: string)
  getDynamicGlobalProperties(): Promise<DynamicGlobalProperties>
  broadcastCustomJson(customJson: CustomJson, key: string): Promise<void>
}
