use super::{
    cargo::CargoTomlError, crystal::ShardYmlError, gleam::GleamTomlError, lerna::LernaJsonError,
    npm::PackageJsonError, pubspec::PubspecYamlError, pyproject::PyprojectTomlError,
};

#[derive(Debug)]
pub enum PackageManagerError {
    CargoToml(CargoTomlError),
    LernaJson(LernaJsonError),
    GleamToml(GleamTomlError),
    PackageJson(PackageJsonError),
    PubspecYaml(PubspecYamlError),
    PyprojectToml(PyprojectTomlError),
    ShardYml(ShardYmlError),
}

impl core::error::Error for PackageManagerError {}

impl core::fmt::Display for PackageManagerError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CargoToml(error) => error.fmt(f),
            Self::LernaJson(error) => error.fmt(f),
            Self::GleamToml(error) => error.fmt(f),
            Self::PackageJson(error) => error.fmt(f),
            Self::PubspecYaml(error) => error.fmt(f),
            Self::PyprojectToml(error) => error.fmt(f),
            Self::ShardYml(error) => error.fmt(f),
        }
    }
}

impl From<CargoTomlError> for PackageManagerError {
    #[inline]
    fn from(value: CargoTomlError) -> Self {
        Self::CargoToml(value)
    }
}

impl From<ShardYmlError> for PackageManagerError {
    #[inline]
    fn from(value: ShardYmlError) -> Self {
        Self::ShardYml(value)
    }
}

impl From<PubspecYamlError> for PackageManagerError {
    #[inline]
    fn from(value: PubspecYamlError) -> Self {
        Self::PubspecYaml(value)
    }
}

impl From<PyprojectTomlError> for PackageManagerError {
    #[inline]
    fn from(value: PyprojectTomlError) -> Self {
        Self::PyprojectToml(value)
    }
}

impl From<PackageJsonError> for PackageManagerError {
    #[inline]
    fn from(value: PackageJsonError) -> Self {
        Self::PackageJson(value)
    }
}

impl From<GleamTomlError> for PackageManagerError {
    #[inline]
    fn from(value: GleamTomlError) -> Self {
        Self::GleamToml(value)
    }
}

impl From<LernaJsonError> for PackageManagerError {
    #[inline]
    fn from(value: LernaJsonError) -> Self {
        Self::LernaJson(value)
    }
}
