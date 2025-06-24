// https://developer.apple.com/documentation/healthkit/data-types
// TODO: data type definitions are incomplete
#[derive(Debug, PartialEq, strum::EnumString)]
#[strum(
    parse_err_fn = handle_parse_error,
    parse_err_ty = anyhow::Error,
)]
pub enum RecordType {
    #[strum(
        serialize = "HKQuantityTypeIdentifierActiveEnergyBurned",
        serialize = "ActiveEnergyBurned"
    )]
    ActiveEnergyBurned,
    #[strum(
        serialize = "HKQuantityTypeIdentifierAppleExerciseTime",
        serialize = "AppleExerciseTime"
    )]
    AppleExerciseTime,
    #[strum(
        serialize = "HKQuantityTypeIdentifierAppleSleepingWristTemperature",
        serialize = "AppleSleepingWristTemperature"
    )]
    AppleSleepingWristTemperature,
    #[strum(
        serialize = "HKCategoryTypeIdentifierAppleStandHour",
        serialize = "AppleStandHour"
    )]
    AppleStandHour,
    #[strum(
        serialize = "HKQuantityTypeIdentifierAppleStandTime",
        serialize = "AppleStandTime"
    )]
    AppleStandTime,
    #[strum(
        serialize = "HKQuantityTypeIdentifierAppleWalkingSteadiness",
        serialize = "AppleWalkingSteadiness"
    )]
    AppleWalkingSteadiness,
    #[strum(
        serialize = "HKCategoryTypeIdentifierAudioExposureEvent",
        serialize = "AudioExposureEvent"
    )]
    AudioExposureEvent,
    #[strum(
        serialize = "HKQuantityTypeIdentifierBasalEnergyBurned",
        serialize = "BasalEnergyBurned"
    )]
    BasalEnergyBurned,
    #[strum(
        serialize = "HKQuantityTypeIdentifierBodyFatPercentage",
        serialize = "BodyFatPercentage"
    )]
    BodyFatPercentage,
    #[strum(serialize = "HKQuantityTypeIdentifierBodyMass", serialize = "BodyMass")]
    BodyMass,
    #[strum(
        serialize = "HKQuantityTypeIdentifierBodyMassIndex",
        serialize = "BodyMassIndex"
    )]
    BodyMassIndex,
    #[strum(
        serialize = "HKQuantityTypeIdentifierBodyTemperature",
        serialize = "BodyTemperature"
    )]
    BodyTemperature,
    #[strum(
        serialize = "HKQuantityTypeIdentifierDietaryEnergyConsumed",
        serialize = "DietaryEnergyConsumed"
    )]
    DietaryEnergyConsumed,
    #[strum(
        serialize = "HKQuantityTypeIdentifierDietaryWater",
        serialize = "DietaryWater"
    )]
    DietaryWater,
    #[strum(
        serialize = "HKQuantityTypeIdentifierDistanceCycling",
        serialize = "DistanceCycling"
    )]
    DistanceCycling,
    #[strum(
        serialize = "HKQuantityTypeIdentifierDistanceSwimming",
        serialize = "DistanceSwimming"
    )]
    DistanceSwimming,
    #[strum(
        serialize = "HKQuantityTypeIdentifierDistanceWalkingRunning",
        serialize = "DistanceWalkingRunning"
    )]
    DistanceWalkingRunning,
    #[strum(
        serialize = "HKQuantityTypeIdentifierEnvironmentalAudioExposure",
        serialize = "EnvironmentalAudioExposure"
    )]
    EnvironmentalAudioExposure,
    #[strum(
        serialize = "HKQuantityTypeIdentifierEnvironmentalSoundReduction",
        serialize = "EnvironmentalSoundReduction"
    )]
    EnvironmentalSoundReduction,
    #[strum(
        serialize = "HKQuantityTypeIdentifierFlightsClimbed",
        serialize = "FlightsClimbed"
    )]
    FlightsClimbed,
    #[strum(
        serialize = "HKCategoryTypeIdentifierHandwashingEvent",
        serialize = "HandwashingEvent"
    )]
    HandwashingEvent,
    #[strum(
        serialize = "HKQuantityTypeIdentifierHeadphoneAudioExposure",
        serialize = "HeadphoneAudioExposure"
    )]
    HeadphoneAudioExposure,
    #[strum(
        serialize = "HKQuantityTypeIdentifierHeartRate",
        serialize = "HeartRate"
    )]
    HeartRate,
    #[strum(
        serialize = "HKQuantityTypeIdentifierHeartRateRecoveryOneMinute",
        serialize = "HeartRateRecoveryOneMinute"
    )]
    HeartRateRecoveryOneMinute,
    #[strum(
        serialize = "HKQuantityTypeIdentifierHeartRateVariabilitySDNN",
        serialize = "HeartRateVariabilitySDNN"
    )]
    HeartRateVariabilitySDNN,
    #[strum(serialize = "HKQuantityTypeIdentifierHeight", serialize = "Height")]
    Height,
    #[strum(
        serialize = "HKCategoryTypeIdentifierHighHeartRateEvent",
        serialize = "HighHeartRateEvent"
    )]
    HighHeartRateEvent,
    #[strum(
        serialize = "HKQuantityTypeIdentifierLeanBodyMass",
        serialize = "LeanBodyMass"
    )]
    LeanBodyMass,
    #[strum(
        serialize = "HKCategoryTypeIdentifierMindfulSession",
        serialize = "MindfulSession"
    )]
    MindfulSession,
    #[strum(
        serialize = "HKQuantityTypeIdentifierOxygenSaturation",
        serialize = "OxygenSaturation"
    )]
    OxygenSaturation,
    #[strum(
        serialize = "HKQuantityTypeIdentifierPhysicalEffort",
        serialize = "PhysicalEffort"
    )]
    PhysicalEffort,
    #[strum(
        serialize = "HKQuantityTypeIdentifierRespiratoryRate",
        serialize = "RespiratoryRate"
    )]
    RespiratoryRate,
    #[strum(
        serialize = "HKQuantityTypeIdentifierRestingHeartRate",
        serialize = "RestingHeartRate"
    )]
    RestingHeartRate,
    #[strum(
        serialize = "HKCategoryTypeIdentifierSexualActivity",
        serialize = "SexualActivity"
    )]
    SexualActivity,
    #[strum(
        serialize = "HKQuantityTypeIdentifierSixMinuteWalkTestDistance",
        serialize = "SixMinuteWalkTestDistance"
    )]
    SixMinuteWalkTestDistance,
    #[strum(
        serialize = "HKCategoryTypeIdentifierSleepAnalysis",
        serialize = "SleepAnalysis"
    )]
    SleepAnalysis,
    #[strum(
        serialize = "HKDataTypeSleepDurationGoal",
        serialize = "SleepDurationGoal"
    )]
    SleepDurationGoal,
    #[strum(
        serialize = "HKQuantityTypeIdentifierStairAscentSpeed",
        serialize = "StairAscentSpeed"
    )]
    StairAscentSpeed,
    #[strum(
        serialize = "HKQuantityTypeIdentifierStairDescentSpeed",
        serialize = "StairDescentSpeed"
    )]
    StairDescentSpeed,
    #[strum(
        serialize = "HKQuantityTypeIdentifierStepCount",
        serialize = "StepCount"
    )]
    StepCount,
    #[strum(
        serialize = "HKQuantityTypeIdentifierSwimmingStrokeCount",
        serialize = "SwimmingStrokeCount"
    )]
    SwimmingStrokeCount,
    #[strum(
        serialize = "HKQuantityTypeIdentifierTimeInDaylight",
        serialize = "TimeInDaylight"
    )]
    TimeInDaylight,
    #[strum(serialize = "HKQuantityTypeIdentifierVO2Max", serialize = "VO2Max")]
    VO2Max,
    #[strum(
        serialize = "HKQuantityTypeIdentifierWalkingAsymmetryPercentage",
        serialize = "WalkingAsymmetryPercentage"
    )]
    WalkingAsymmetryPercentage,
    #[strum(
        serialize = "HKQuantityTypeIdentifierWalkingDoubleSupportPercentage",
        serialize = "WalkingDoubleSupportPercentage"
    )]
    WalkingDoubleSupportPercentage,
    #[strum(
        serialize = "HKQuantityTypeIdentifierWalkingHeartRateAverage",
        serialize = "WalkingHeartRateAverage"
    )]
    WalkingHeartRateAverage,
    #[strum(
        serialize = "HKQuantityTypeIdentifierWalkingSpeed",
        serialize = "WalkingSpeed"
    )]
    WalkingSpeed,
    #[strum(
        serialize = "HKQuantityTypeIdentifierWalkingStepLength",
        serialize = "WalkingStepLength"
    )]
    WalkingStepLength,
    #[strum(
        serialize = "HKQuantityTypeIdentifierWaterTemperature",
        serialize = "WaterTemperature"
    )]
    WaterTemperature,
}

fn handle_parse_error(err: &str) -> anyhow::Error {
    anyhow::Error::msg(format!("failed to parse record type: {}", err))
}
