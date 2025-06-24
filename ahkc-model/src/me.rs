/// Personal characteristics stored in Health app
///
/// # Attributes
/// - `HKCharacteristicTypeIdentifierDateOfBirth`: Birth date in YYYY-MM-DD format
/// - `HKCharacteristicTypeIdentifierBiologicalSex`: Sex (e.g., "HKBiologicalSexMale", "HKBiologicalSexFemale")
/// - `HKCharacteristicTypeIdentifierBloodType`: Blood type (e.g., "HKBloodTypeAPositive", "HKBloodTypeNotSet")
/// - `HKCharacteristicTypeIdentifierFitzpatrickSkinType`: Skin type for UV exposure
/// - `HKCharacteristicTypeIdentifierCardioFitnessMedicationsUse`: Medication use status
#[derive(Debug)]
pub struct Me {
    pub date_of_birth: String,
    pub biological_sex: String,
    pub blood_type: String,
    pub fitzpatrick_skin_type: String,
    pub cardio_fitness_medications_use: String,
}
