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

import { BuildVersion } from '../models/BuildVersion';
import { HttpFile } from '../http/http';

/**
* algod version information.
*/
export class Version {
    'build': BuildVersion;
    'genesisHashB64': string;
    'genesisId': string;
    'versions': Array<string>;

    static readonly discriminator: string | undefined = undefined;

    static readonly mapping: {[index: string]: string} | undefined = undefined;

    static readonly attributeTypeMap: Array<{name: string, baseName: string, type: string, format: string}> = [
        {
            "name": "build",
            "baseName": "build",
            "type": "BuildVersion",
            "format": ""
        },
        {
            "name": "genesisHashB64",
            "baseName": "genesis_hash_b64",
            "type": "string",
            "format": "byte"
        },
        {
            "name": "genesisId",
            "baseName": "genesis_id",
            "type": "string",
            "format": ""
        },
        {
            "name": "versions",
            "baseName": "versions",
            "type": "Array<string>",
            "format": ""
        }    ];

    static getAttributeTypeMap() {
        return Version.attributeTypeMap;
    }

    public constructor() {
    }
}
