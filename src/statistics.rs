//! MDATP JSON statistics models.

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Deserialize)]
pub struct Statistics {
    pub counters: Vec<Counter>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct Counter {
    #[serde(rename = "id")]
    pub process_id: u32,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub path: Option<String>,

    #[serde(rename = "isActive")]
    pub is_active: bool,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "maxFileScanTime")]
    pub max_file_scan_time: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "resourceScanTime")]
    pub resource_scan_time: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "totalFilesScanned", alias = "total_files_scanned")]
    pub n_total_files_scanned: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "totalScanTime")]
    pub total_scan_time: u64,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_parse_ok() -> Result<()> {
        // language=json
        let input = r#"
            {
              "counters": [
                {
                  "id": 1,
                  "isActive": true,
                  "maxFileScanTime": "143635500",
                  "name": "launchd",
                  "path": "/sbin/launchd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2071",
                  "totalScanTime": "35706172777"
                },
                {
                  "id": 533,
                  "isActive": true,
                  "maxFileScanTime": "20341042",
                  "name": "configd",
                  "path": "/usr/libexec/configd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "5",
                  "totalScanTime": "32638292"
                },
                {
                  "id": 535,
                  "isActive": true,
                  "maxFileScanTime": "49298625",
                  "name": "powerd",
                  "path": "/System/Library/CoreServices/powerd.bundle/powerd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "68",
                  "totalScanTime": "857069749"
                },
                {
                  "id": 537,
                  "isActive": true,
                  "maxFileScanTime": "54918916",
                  "name": "biomed",
                  "path": "/System/Library/PrivateFrameworks/BiomeStreams.framework/Support/biomed",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "312",
                  "totalScanTime": "2218684374"
                },
                {
                  "id": 561,
                  "isActive": true,
                  "maxFileScanTime": "33397500",
                  "name": "thermalmonitord",
                  "path": "/usr/libexec/thermalmonitord",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "14",
                  "totalScanTime": "185325913"
                },
                {
                  "id": 562,
                  "isActive": true,
                  "maxFileScanTime": "56612417",
                  "name": "opendirectoryd",
                  "path": "/usr/libexec/opendirectoryd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "568",
                  "totalScanTime": "1347772895"
                },
                {
                  "id": 563,
                  "isActive": true,
                  "maxFileScanTime": "51586750",
                  "name": "apsd",
                  "path": "/System/Library/PrivateFrameworks/ApplePushService.framework/apsd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "56",
                  "totalScanTime": "574258748"
                },
                {
                  "id": 565,
                  "isActive": true,
                  "maxFileScanTime": "94131958",
                  "name": "timed",
                  "path": "/usr/libexec/timed",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "21",
                  "totalScanTime": "396544414"
                },
                {
                  "id": 566,
                  "isActive": true,
                  "maxFileScanTime": "57083",
                  "name": "usbmuxd",
                  "path": "/Library/Apple/System/Library/PrivateFrameworks/MobileDevice.framework/Versions/A/Resources/usbmuxd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2",
                  "totalScanTime": "101333"
                },
                {
                  "id": 567,
                  "isActive": true,
                  "maxFileScanTime": "116584",
                  "name": "securityd",
                  "path": "/usr/sbin/securityd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "1",
                  "totalScanTime": "116584"
                },
                {
                  "id": 572,
                  "isActive": true,
                  "maxFileScanTime": "37239750",
                  "name": "dasd",
                  "path": "/usr/libexec/dasd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "16",
                  "totalScanTime": "129213667"
                },
                {
                  "id": 584,
                  "isActive": true,
                  "maxFileScanTime": "7308167",
                  "name": "notifyd",
                  "path": "/usr/sbin/notifyd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "1",
                  "totalScanTime": "7308167"
                },
                {
                  "id": 585,
                  "isActive": true,
                  "maxFileScanTime": "32886709",
                  "name": "sandboxd",
                  "path": "/usr/libexec/sandboxd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "8",
                  "totalScanTime": "73637834"
                },
                {
                  "id": 590,
                  "isActive": true,
                  "maxFileScanTime": "5194375",
                  "name": "WindowServer",
                  "path": "/System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/Resources/WindowServer",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "3",
                  "totalScanTime": "13919667"
                },
                {
                  "id": 592,
                  "isActive": true,
                  "maxFileScanTime": "110455917",
                  "name": "tccd",
                  "path": "/System/Library/PrivateFrameworks/TCC.framework/Support/tccd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "53",
                  "totalScanTime": "945107288"
                },
                {
                  "id": 597,
                  "isActive": true,
                  "maxFileScanTime": "41579250",
                  "name": "lsd",
                  "path": "/usr/libexec/lsd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "33",
                  "totalScanTime": "112553038"
                },
                {
                  "id": 607,
                  "isActive": true,
                  "maxFileScanTime": "124962833",
                  "name": "runningboardd",
                  "path": "/usr/libexec/runningboardd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "478",
                  "totalScanTime": "5452612376"
                },
                {
                  "id": 619,
                  "isActive": true,
                  "maxFileScanTime": "468641000",
                  "name": "analyticsd",
                  "path": "/System/Library/PrivateFrameworks/CoreAnalytics.framework/Support/analyticsd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "267",
                  "totalScanTime": "2833781326"
                },
                {
                  "id": 634,
                  "isActive": true,
                  "maxFileScanTime": "91792",
                  "name": "nsurlsessiond",
                  "path": "/usr/libexec/nsurlsessiond",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "3",
                  "totalScanTime": "199917"
                },
                {
                  "id": 638,
                  "isActive": true,
                  "maxFileScanTime": "188327708",
                  "name": "airportd",
                  "path": "/usr/libexec/airportd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "29",
                  "totalScanTime": "1501118250"
                },
                {
                  "id": 664,
                  "isActive": true,
                  "maxFileScanTime": "51366375",
                  "name": "socketfilterfw",
                  "path": "/usr/libexec/ApplicationFirewall/socketfilterfw",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "12",
                  "totalScanTime": "332278542"
                },
                {
                  "id": 677,
                  "isActive": true,
                  "maxFileScanTime": "58642292",
                  "name": "authd",
                  "path": "/System/Library/Frameworks/Security.framework/Versions/A/XPCServices/authd.xpc/Contents/MacOS/authd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "3",
                  "totalScanTime": "125930001"
                },
                {
                  "id": 681,
                  "isActive": true,
                  "maxFileScanTime": "3497791",
                  "name": "com.apple.CodeSigningHelper",
                  "path": "/System/Library/Frameworks/Security.framework/Versions/A/XPCServices/com.apple.CodeSigningHelper.xpc/Contents/MacOS/com.apple.CodeSigningHelper",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2",
                  "totalScanTime": "6457333"
                },
                {
                  "id": 683,
                  "isActive": true,
                  "maxFileScanTime": "20288209",
                  "name": "mDNSResponder",
                  "path": "/usr/sbin/mDNSResponder",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "7",
                  "totalScanTime": "112448417"
                },
                {
                  "id": 684,
                  "isActive": true,
                  "maxFileScanTime": "46741625",
                  "name": "symptomsd",
                  "path": "/usr/libexec/symptomsd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "10",
                  "totalScanTime": "108184208"
                },
                {
                  "id": 829,
                  "isActive": true,
                  "maxFileScanTime": "108864000",
                  "name": "locationd",
                  "path": "/usr/libexec/locationd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "485",
                  "totalScanTime": "2394697919"
                },
                {
                  "id": 834,
                  "isActive": true,
                  "maxFileScanTime": "54276000",
                  "name": "lghub_updater",
                  "path": "/Applications/lghub.app/Contents/MacOS/lghub_updater.app/Contents/MacOS/lghub_updater",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "55",
                  "totalScanTime": "517737585"
                },
                {
                  "id": 980,
                  "isActive": true,
                  "maxFileScanTime": "18606167",
                  "name": "lsd",
                  "path": "/usr/libexec/lsd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "46",
                  "totalScanTime": "180121792"
                },
                {
                  "id": 1013,
                  "isActive": true,
                  "maxFileScanTime": "9242875",
                  "name": "mdbulkimport",
                  "path": "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Support/mdbulkimport",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "274",
                  "totalScanTime": "543304352"
                },
                {
                  "id": 1014,
                  "isActive": true,
                  "maxFileScanTime": "13385542",
                  "name": "mdbulkimport",
                  "path": "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Support/mdbulkimport",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "427",
                  "totalScanTime": "854187241"
                },
                {
                  "id": 1025,
                  "isActive": true,
                  "maxFileScanTime": "76900916",
                  "name": "pkd",
                  "path": "/usr/libexec/pkd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "22",
                  "totalScanTime": "559649335"
                },
                {
                  "id": 1063,
                  "isActive": true,
                  "maxFileScanTime": "249542",
                  "name": "oahd",
                  "path": "/usr/libexec/rosetta/oahd",
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "28",
                  "totalScanTime": "2859504"
                },
                {
                  "id": 1211,
                  "isActive": true,
                  "maxFileScanTime": "9561667",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "1661",
                  "totalScanTime": "134244602"
                },
                {
                  "id": 1214,
                  "isActive": true,
                  "maxFileScanTime": "30243166",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2",
                  "totalScanTime": "49604083"
                },
                {
                  "id": 1224,
                  "isActive": true,
                  "maxFileScanTime": "53204625",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "51",
                  "totalScanTime": "157857422"
                },
                {
                  "id": 1226,
                  "isActive": true,
                  "maxFileScanTime": "32890833",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "63",
                  "totalScanTime": "692135330"
                },
                {
                  "id": 1227,
                  "isActive": true,
                  "maxFileScanTime": "18173083",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "5",
                  "totalScanTime": "38522333"
                },
                {
                  "id": 1230,
                  "isActive": true,
                  "maxFileScanTime": "101781542",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "90",
                  "totalScanTime": "1314467340"
                },
                {
                  "id": 1239,
                  "isActive": true,
                  "maxFileScanTime": "32523250",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "4",
                  "totalScanTime": "83606333"
                },
                {
                  "id": 1242,
                  "isActive": true,
                  "maxFileScanTime": "131218375",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "25",
                  "totalScanTime": "784982960"
                },
                {
                  "id": 1243,
                  "isActive": true,
                  "maxFileScanTime": "98695541",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "71",
                  "totalScanTime": "565039579"
                },
                {
                  "id": 1249,
                  "isActive": true,
                  "maxFileScanTime": "23443375",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "12",
                  "totalScanTime": "95242666"
                },
                {
                  "id": 1251,
                  "isActive": true,
                  "maxFileScanTime": "33486167",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "5",
                  "totalScanTime": "107981376"
                },
                {
                  "id": 1263,
                  "isActive": true,
                  "maxFileScanTime": "103280458",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "12",
                  "totalScanTime": "370366916"
                },
                {
                  "id": 1291,
                  "isActive": true,
                  "maxFileScanTime": "60466667",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "205",
                  "totalScanTime": "894196132"
                },
                {
                  "id": 1301,
                  "isActive": true,
                  "maxFileScanTime": "109078792",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "5516",
                  "totalScanTime": "20753049150"
                },
                {
                  "id": 1324,
                  "isActive": true,
                  "maxFileScanTime": "18513333",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "6",
                  "totalScanTime": "63277250"
                },
                {
                  "id": 1915,
                  "isActive": true,
                  "maxFileScanTime": "62245208",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "4",
                  "totalScanTime": "66710833"
                },
                {
                  "id": 1924,
                  "isActive": true,
                  "maxFileScanTime": "729909833",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "287",
                  "totalScanTime": "32525087671"
                },
                {
                  "id": 1925,
                  "isActive": true,
                  "maxFileScanTime": "114424042",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "141",
                  "totalScanTime": "5024014918"
                },
                {
                  "id": 1926,
                  "isActive": true,
                  "maxFileScanTime": "22715334",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "11",
                  "totalScanTime": "104423044"
                },
                {
                  "id": 1927,
                  "isActive": true,
                  "maxFileScanTime": "102282750",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "1",
                  "totalScanTime": "102282750"
                },
                {
                  "id": 1928,
                  "isActive": true,
                  "maxFileScanTime": "21445709",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "140",
                  "totalScanTime": "603748713"
                },
                {
                  "id": 1980,
                  "isActive": true,
                  "maxFileScanTime": "150143250",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "365",
                  "totalScanTime": "10416026808"
                },
                {
                  "id": 1988,
                  "isActive": true,
                  "maxFileScanTime": "27712917",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "14",
                  "totalScanTime": "126796210"
                },
                {
                  "id": 2015,
                  "isActive": true,
                  "maxFileScanTime": "90484000",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "172",
                  "totalScanTime": "1729482134"
                },
                {
                  "id": 2047,
                  "isActive": true,
                  "maxFileScanTime": "121780042",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "97",
                  "totalScanTime": "787193625"
                },
                {
                  "id": 4238,
                  "isActive": true,
                  "maxFileScanTime": "31129208",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "55",
                  "totalScanTime": "443351164"
                },
                {
                  "id": 4250,
                  "isActive": true,
                  "maxFileScanTime": "51733458",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "88",
                  "totalScanTime": "1154199292"
                },
                {
                  "id": 4342,
                  "isActive": true,
                  "maxFileScanTime": "201956333",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "55609",
                  "totalScanTime": "1214122595563"
                },
                {
                  "id": 4389,
                  "isActive": true,
                  "maxFileScanTime": "109415666",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "32549",
                  "totalScanTime": "67479739180"
                },
                {
                  "id": 4390,
                  "isActive": true,
                  "maxFileScanTime": "20659083",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "21",
                  "totalScanTime": "141560751"
                },
                {
                  "id": 6495,
                  "isActive": true,
                  "maxFileScanTime": "29628584",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "4",
                  "totalScanTime": "42053167"
                },
                {
                  "id": 6499,
                  "isActive": true,
                  "maxFileScanTime": "18807000",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "10",
                  "totalScanTime": "102678417"
                },
                {
                  "id": 6625,
                  "isActive": true,
                  "maxFileScanTime": "117052833",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "6",
                  "totalScanTime": "177085999"
                },
                {
                  "id": 6731,
                  "isActive": true,
                  "maxFileScanTime": "25808458",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2",
                  "totalScanTime": "33588333"
                },
                {
                  "id": 6732,
                  "isActive": true,
                  "maxFileScanTime": "19162792",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "366",
                  "totalScanTime": "683032576"
                },
                {
                  "id": 7851,
                  "isActive": true,
                  "maxFileScanTime": "22714167",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "31",
                  "totalScanTime": "244420124"
                },
                {
                  "id": 7854,
                  "isActive": true,
                  "maxFileScanTime": "208041",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "501",
                  "totalScanTime": "7227428"
                },
                {
                  "id": 8273,
                  "isActive": true,
                  "maxFileScanTime": "21143417",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "60",
                  "totalScanTime": "354496373"
                },
                {
                  "id": 8371,
                  "isActive": true,
                  "maxFileScanTime": "51407958",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "26",
                  "totalScanTime": "268538750"
                },
                {
                  "id": 8717,
                  "isActive": true,
                  "maxFileScanTime": "13875000",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "4",
                  "totalScanTime": "36622626"
                },
                {
                  "id": 9051,
                  "isActive": true,
                  "maxFileScanTime": "31863083",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "7",
                  "totalScanTime": "115468707"
                },
                {
                  "id": 10189,
                  "isActive": true,
                  "maxFileScanTime": "9535875",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2",
                  "totalScanTime": "14563625"
                },
                {
                  "id": 11560,
                  "isActive": true,
                  "maxFileScanTime": "51669000",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "28",
                  "totalScanTime": "415689915"
                },
                {
                  "id": 12046,
                  "isActive": true,
                  "maxFileScanTime": "28933292",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "10",
                  "totalScanTime": "160655001"
                },
                {
                  "id": 20300,
                  "isActive": true,
                  "maxFileScanTime": "29201292",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "5",
                  "totalScanTime": "114012334"
                },
                {
                  "id": 25858,
                  "isActive": true,
                  "maxFileScanTime": "646202042",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "11013",
                  "totalScanTime": "55898684270"
                },
                {
                  "id": 26924,
                  "isActive": true,
                  "maxFileScanTime": "32553250",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "1",
                  "totalScanTime": "32553250"
                },
                {
                  "id": 41321,
                  "isActive": true,
                  "maxFileScanTime": "35317875",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "89",
                  "totalScanTime": "1180315578"
                },
                {
                  "id": 42231,
                  "isActive": true,
                  "maxFileScanTime": "31181916",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "11",
                  "totalScanTime": "205422249"
                },
                {
                  "id": 54225,
                  "isActive": true,
                  "maxFileScanTime": "6428167",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "4",
                  "totalScanTime": "19467334"
                },
                {
                  "id": 61158,
                  "isActive": true,
                  "maxFileScanTime": "32744459",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "59",
                  "totalScanTime": "676206251"
                },
                {
                  "id": 61159,
                  "isActive": true,
                  "maxFileScanTime": "18593792",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "14",
                  "totalScanTime": "102857710"
                },
                {
                  "id": 61174,
                  "isActive": true,
                  "maxFileScanTime": "1174208",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "114",
                  "totalScanTime": "20174843"
                },
                {
                  "id": 65842,
                  "isActive": true,
                  "maxFileScanTime": "128364125",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "817",
                  "totalScanTime": "10342038165"
                },
                {
                  "id": 70403,
                  "isActive": true,
                  "maxFileScanTime": "6319333",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "3",
                  "totalScanTime": "6405291"
                },
                {
                  "id": 76190,
                  "isActive": true,
                  "maxFileScanTime": "58757000",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "65",
                  "totalScanTime": "683593169"
                },
                {
                  "id": 76207,
                  "isActive": true,
                  "maxFileScanTime": "133443417",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "154",
                  "totalScanTime": "6252776628"
                },
                {
                  "id": 76220,
                  "isActive": true,
                  "maxFileScanTime": "13984166",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "6",
                  "totalScanTime": "51605458"
                },
                {
                  "id": 76248,
                  "isActive": true,
                  "maxFileScanTime": "70737208",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "450",
                  "totalScanTime": "4515512166"
                },
                {
                  "id": 84630,
                  "isActive": true,
                  "maxFileScanTime": "81935875",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "616",
                  "totalScanTime": "5539493087"
                },
                {
                  "id": 84645,
                  "isActive": true,
                  "maxFileScanTime": "175157375",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "13",
                  "totalScanTime": "250352627"
                },
                {
                  "id": 88466,
                  "isActive": true,
                  "maxFileScanTime": "108169125",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "362",
                  "totalScanTime": "3511181214"
                },
                {
                  "id": 88492,
                  "isActive": true,
                  "maxFileScanTime": "14387500",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "2",
                  "totalScanTime": "17850917"
                },
                {
                  "id": 88913,
                  "isActive": true,
                  "maxFileScanTime": "29842208",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "164",
                  "totalScanTime": "1334528078"
                },
                {
                  "id": 91040,
                  "isActive": true,
                  "maxFileScanTime": "21083375",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "22",
                  "totalScanTime": "187002666"
                },
                {
                  "id": 91041,
                  "isActive": true,
                  "maxFileScanTime": "107275000",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "55",
                  "totalScanTime": "937481753"
                },
                {
                  "id": 91056,
                  "isActive": true,
                  "maxFileScanTime": "15027542",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "4",
                  "totalScanTime": "15250833"
                },
                {
                  "id": 91072,
                  "isActive": true,
                  "maxFileScanTime": "51767875",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "56",
                  "totalScanTime": "813085415"
                },
                {
                  "id": 91470,
                  "isActive": true,
                  "maxFileScanTime": "3785417",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "26",
                  "totalScanTime": "9682172"
                },
                {
                  "id": 91498,
                  "isActive": true,
                  "maxFileScanTime": "22552458",
                  "name": null,
                  "path": null,
                  "resourceScanTime": "0",
                  "scannedFilePaths": null,
                  "totalEventsSent": "0",
                  "totalFilesScanned": "1",
                  "totalScanTime": "22552458"
                }
              ]
            }
        "#;
        serde_json::from_str::<Statistics>(input)?;
        Ok(())
    }
}
