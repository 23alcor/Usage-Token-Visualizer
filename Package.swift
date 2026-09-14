// swift-tools-version: 5.10

import PackageDescription

let package = Package(
    name: "TokenUsage",
    platforms: [
        .macOS(.v14),
        .iOS(.v17)
    ],
    products: [
        .library(name: "TokenUsageCore", targets: ["TokenUsageCore"])
    ],
    targets: [
        .target(name: "TokenUsageCore"),
        .testTarget(name: "TokenUsageCoreTests", dependencies: ["TokenUsageCore"])
    ]
)

