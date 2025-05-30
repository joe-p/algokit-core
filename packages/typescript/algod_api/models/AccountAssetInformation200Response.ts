/**
 * Algod REST API.
 * API endpoint for algod operations.
 *
 * OpenAPI spec version: 0.0.1
 * Contact: contact@algorand.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { AssetHolding } from '../models/AssetHolding';
import { AssetParams } from '../models/AssetParams';
import { HttpFile } from '../http/http';

export class AccountAssetInformation200Response {
    /**
    * The round for which this information is relevant.
    */
    'round': number;
    'assetHolding'?: AssetHolding;
    'createdAsset'?: AssetParams;

    static readonly discriminator: string | undefined = undefined;

    static readonly mapping: {[index: string]: string} | undefined = undefined;

    static readonly attributeTypeMap: Array<{name: string, baseName: string, type: string, format: string}> = [
        {
            "name": "round",
            "baseName": "round",
            "type": "number",
            "format": ""
        },
        {
            "name": "assetHolding",
            "baseName": "asset-holding",
            "type": "AssetHolding",
            "format": ""
        },
        {
            "name": "createdAsset",
            "baseName": "created-asset",
            "type": "AssetParams",
            "format": ""
        }    ];

    static getAttributeTypeMap() {
        return AccountAssetInformation200Response.attributeTypeMap;
    }

    public constructor() {
    }
}
