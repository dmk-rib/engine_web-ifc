//! Auto-translated property name/type tables from C++ schema-names.h.

#![allow(dead_code)]

use super::ifc_schema::IFC_SCHEMA;

pub const PROPY_NAMES: [&str; 1215] = [
    "Role",
    "UserDefinedRole",
    "Description",
    "Purpose",
    "UserDefinedPurpose",
    "ApplicationDeveloper",
    "Version",
    "ApplicationFullName",
    "ApplicationIdentifier",
    "Name",
    "AppliedValue",
    "UnitBasis",
    "ApplicableDate",
    "FixedUntilDate",
    "ComponentOfTotal",
    "Components",
    "ArithmeticOperator",
    "ApprovalDateTime",
    "ApprovalStatus",
    "ApprovalLevel",
    "ApprovalQualifier",
    "Identifier",
    "Actor",
    "Approval",
    "ApprovedProperties",
    "RelatedApproval",
    "RelatingApproval",
    "LinearStiffnessByLengthX",
    "LinearStiffnessByLengthY",
    "LinearStiffnessByLengthZ",
    "RotationalStiffnessByLengthX",
    "RotationalStiffnessByLengthY",
    "RotationalStiffnessByLengthZ",
    "LinearStiffnessByAreaX",
    "LinearStiffnessByAreaY",
    "LinearStiffnessByAreaZ",
    "LinearStiffnessX",
    "LinearStiffnessY",
    "LinearStiffnessZ",
    "RotationalStiffnessX",
    "RotationalStiffnessY",
    "RotationalStiffnessZ",
    "WarpingStiffness",
    "DayComponent",
    "MonthComponent",
    "YearComponent",
    "Source",
    "Edition",
    "EditionDate",
    "Notation",
    "ItemOf",
    "Title",
    "RelatingItem",
    "RelatedItems",
    "NotationFacets",
    "NotationValue",
    "PointOnRelatingElement",
    "PointOnRelatedElement",
    "LocationAtRelatingElement",
    "LocationAtRelatedElement",
    "ProfileOfPort",
    "SurfaceOnRelatingElement",
    "SurfaceOnRelatedElement",
    "ConstraintGrade",
    "ConstraintSource",
    "CreatingActor",
    "CreationTime",
    "UserDefinedGrade",
    "RelatingConstraint",
    "RelatedConstraints",
    "LogicalAggregator",
    "ClassifiedConstraint",
    "RelatedClassifications",
    "HourOffset",
    "MinuteOffset",
    "Sense",
    "CostType",
    "Condition",
    "RelatingMonetaryUnit",
    "RelatedMonetaryUnit",
    "ExchangeRate",
    "RateDateTime",
    "RateSource",
    "PatternList",
    "CurveFont",
    "CurveFontScaling",
    "VisibleSegmentLength",
    "InvisibleSegmentLength",
    "DateComponent",
    "TimeComponent",
    "Elements",
    "UnitType",
    "UserDefinedType",
    "Unit",
    "Exponent",
    "LengthExponent",
    "MassExponent",
    "TimeExponent",
    "ElectricCurrentExponent",
    "ThermodynamicTemperatureExponent",
    "AmountOfSubstanceExponent",
    "LuminousIntensityExponent",
    "FileExtension",
    "MimeContentType",
    "MimeSubtype",
    "DocumentId",
    "DocumentReferences",
    "IntendedUse",
    "Scope",
    "Revision",
    "DocumentOwner",
    "Editors",
    "LastRevisionTime",
    "ElectronicFormat",
    "ValidFrom",
    "ValidUntil",
    "Confidentiality",
    "Status",
    "RelatingDocument",
    "RelatedDocuments",
    "RelationshipType",
    "RelatingDraughtingCallout",
    "RelatedDraughtingCallout",
    "ImpactType",
    "Category",
    "UserDefinedCategory",
    "Location",
    "ItemReference",
    "AxisTag",
    "AxisCurve",
    "SameSense",
    "TimeStamp",
    "ListValues",
    "Publisher",
    "VersionDate",
    "LibraryReference",
    "MainPlaneAngle",
    "SecondaryPlaneAngle",
    "LuminousIntensity",
    "LightDistributionCurve",
    "DistributionData",
    "HourComponent",
    "MinuteComponent",
    "SecondComponent",
    "Zone",
    "DaylightSavingOffset",
    "MaterialClassifications",
    "ClassifiedMaterial",
    "Material",
    "LayerThickness",
    "IsVentilated",
    "MaterialLayers",
    "LayerSetName",
    "ForLayerSet",
    "LayerSetDirection",
    "DirectionSense",
    "OffsetFromReferenceLine",
    "Materials",
    "ValueComponent",
    "UnitComponent",
    "DynamicViscosity",
    "YoungModulus",
    "ShearModulus",
    "PoissonRatio",
    "ThermalExpansionCoefficient",
    "YieldStress",
    "UltimateStress",
    "UltimateStrain",
    "HardeningModule",
    "ProportionalStress",
    "PlasticStrain",
    "Relaxations",
    "Benchmark",
    "ValueSource",
    "DataValue",
    "Currency",
    "Dimensions",
    "BenchmarkValues",
    "ResultValues",
    "ObjectiveQualifier",
    "UserDefinedQualifier",
    "VisibleTransmittance",
    "SolarTransmittance",
    "ThermalIrTransmittance",
    "ThermalIrEmissivityBack",
    "ThermalIrEmissivityFront",
    "VisibleReflectanceBack",
    "VisibleReflectanceFront",
    "SolarReflectanceFront",
    "SolarReflectanceBack",
    "Id",
    "Roles",
    "Addresses",
    "RelatingOrganization",
    "RelatedOrganizations",
    "OwningUser",
    "OwningApplication",
    "State",
    "ChangeAction",
    "LastModifiedDate",
    "LastModifyingUser",
    "LastModifyingApplication",
    "CreationDate",
    "FamilyName",
    "GivenName",
    "MiddleNames",
    "PrefixTitles",
    "SuffixTitles",
    "ThePerson",
    "TheOrganization",
    "InternalLocation",
    "AddressLines",
    "PostalBox",
    "Town",
    "Region",
    "PostalCode",
    "Country",
    "AssignedItems",
    "LayerOn",
    "LayerFrozen",
    "LayerBlocked",
    "LayerStyles",
    "Styles",
    "Representations",
    "SpecificHeatCapacity",
    "N20Content",
    "COContent",
    "CO2Content",
    "ProfileType",
    "ProfileName",
    "ProfileDefinition",
    "RelatedProperties",
    "DependingProperty",
    "DependantProperty",
    "Expression",
    "EnumerationValues",
    "AreaValue",
    "CountValue",
    "LengthValue",
    "TimeValue",
    "VolumeValue",
    "WeightValue",
    "ReferencedDocument",
    "ReferencingValues",
    "TotalCrossSectionArea",
    "SteelGrade",
    "BarSurface",
    "EffectiveDepth",
    "NominalBarDiameter",
    "BarCount",
    "RelaxationValue",
    "InitialStress",
    "ContextOfItems",
    "RepresentationIdentifier",
    "RepresentationType",
    "Items",
    "ContextIdentifier",
    "ContextType",
    "MappingOrigin",
    "MappedRepresentation",
    "Thickness",
    "RibHeight",
    "RibWidth",
    "RibSpacing",
    "Direction",
    "GlobalId",
    "OwnerHistory",
    "Prefix",
    "SectionType",
    "StartProfile",
    "EndProfile",
    "LongitudinalStartPosition",
    "LongitudinalEndPosition",
    "TransversePosition",
    "ReinforcementRole",
    "SectionDefinition",
    "CrossSectionReinforcementDefinitions",
    "ShapeRepresentations",
    "ProductDefinitional",
    "PartOfProductDefinitionShape",
    "DeltaT_Constant",
    "DeltaT_Y",
    "DeltaT_Z",
    "Item",
    "Side",
    "DiffuseTransmissionColour",
    "DiffuseReflectionColour",
    "TransmissionColour",
    "ReflectanceColour",
    "RefractionIndex",
    "DispersionFactor",
    "SurfaceColour",
    "Textures",
    "RepeatS",
    "RepeatT",
    "TextureType",
    "TextureTransform",
    "StyleOfSymbol",
    "Rows",
    "RowCells",
    "IsHeading",
    "TelephoneNumbers",
    "FacsimileNumbers",
    "PagerNumber",
    "ElectronicMailAddresses",
    "WWWHomePageURL",
    "TextCharacterAppearance",
    "TextStyle",
    "TextFontStyle",
    "FontFamily",
    "FontStyle",
    "FontVariant",
    "FontWeight",
    "FontSize",
    "Colour",
    "BackgroundColour",
    "TextIndent",
    "TextAlign",
    "TextDecoration",
    "LetterSpacing",
    "WordSpacing",
    "TextTransform",
    "LineHeight",
    "BoxHeight",
    "BoxWidth",
    "BoxSlantAngle",
    "BoxRotateAngle",
    "CharacterSpacing",
    "Mode",
    "Parameter",
    "TextureMaps",
    "Coordinates",
    "BoilingPoint",
    "FreezingPoint",
    "ThermalConductivity",
    "StartTime",
    "EndTime",
    "TimeSeriesDataType",
    "DataOrigin",
    "UserDefinedDataOrigin",
    "ReferencedTimeSeries",
    "TimeSeriesReferences",
    "Units",
    "TextureVertices",
    "TexturePoints",
    "VertexGeometry",
    "IntersectingAxes",
    "OffsetDistances",
    "IsPotable",
    "Hardness",
    "AlkalinityConcentration",
    "AcidityConcentration",
    "ImpuritiesContent",
    "PHLevel",
    "DissolvedSolidsContent",
    "OuterCurve",
    "Curve",
    "InnerCurves",
    "RasterFormat",
    "RasterCode",
    "ReferencedSource",
    "Red",
    "Green",
    "Blue",
    "UsageName",
    "HasProperties",
    "Profiles",
    "Label",
    "CfsFaces",
    "CurveOnRelatingElement",
    "CurveOnRelatedElement",
    "EccentricityInX",
    "EccentricityInY",
    "EccentricityInZ",
    "ConversionFactor",
    "CurveWidth",
    "CurveColour",
    "ParentProfile",
    "Operator",
    "EdgeStart",
    "EdgeEnd",
    "EdgeGeometry",
    "ExtendedProperties",
    "Bounds",
    "Bound",
    "Orientation",
    "FaceSurface",
    "TensionFailureX",
    "TensionFailureY",
    "TensionFailureZ",
    "CompressionFailureX",
    "CompressionFailureY",
    "CompressionFailureZ",
    "FillStyles",
    "CombustionTemperature",
    "CarbonContent",
    "LowerHeatingValue",
    "HigherHeatingValue",
    "MolecularWeight",
    "Porosity",
    "MassDensity",
    "PhysicalWeight",
    "Perimeter",
    "MinimumPlateThickness",
    "MaximumPlateThickness",
    "CrossSectionArea",
    "CoordinateSpaceDimension",
    "Precision",
    "WorldCoordinateSystem",
    "TrueNorth",
    "ParentContext",
    "TargetScale",
    "TargetView",
    "UserDefinedTargetView",
    "PlacementLocation",
    "PlacementRefDirection",
    "BaseSurface",
    "AgreementFlag",
    "UpperVaporResistanceFactor",
    "LowerVaporResistanceFactor",
    "IsothermalMoistureCapacity",
    "VaporPermeability",
    "MoistureDiffusivity",
    "UrlReference",
    "Values",
    "LightColour",
    "AmbientIntensity",
    "Intensity",
    "Position",
    "ColourAppearance",
    "ColourTemperature",
    "LuminousFlux",
    "LightEmissionSource",
    "LightDistributionDataSource",
    "Radius",
    "ConstantAttenuation",
    "DistanceAttenuation",
    "QuadricAttenuation",
    "ConcentrationExponent",
    "SpreadAngle",
    "BeamWidthAngle",
    "PlacementRelTo",
    "RelativePlacement",
    "MappingSource",
    "MappingTarget",
    "RepresentedMaterial",
    "CompressiveStrength",
    "MaxAggregateSize",
    "AdmixturesDescription",
    "Workability",
    "ProtectivePoreRatio",
    "WaterImpermeability",
    "RepeatFactor",
    "EdgeElement",
    "EdgeList",
    "HasQuantities",
    "Discrimination",
    "Quality",
    "Usage",
    "Width",
    "Height",
    "ColourComponents",
    "Pixel",
    "SizeInX",
    "SizeInY",
    "BasisCurve",
    "PointParameter",
    "BasisSurface",
    "PointParameterU",
    "PointParameterV",
    "Polygon",
    "PolygonalBoundary",
    "UpperBoundValue",
    "LowerBoundValue",
    "EnumerationReference",
    "PropertyReference",
    "NominalValue",
    "DefiningValues",
    "DefinedValues",
    "DefiningUnit",
    "DefinedUnit",
    "XDim",
    "YDim",
    "TimeStep",
    "DefinitionType",
    "ReinforcementSectionDefinitions",
    "RoundingRadius",
    "SpineCurve",
    "CrossSections",
    "CrossSectionPositions",
    "PredefinedType",
    "UpperValue",
    "MostUsedValue",
    "LowerValue",
    "SbsmBoundary",
    "SlippageX",
    "SlippageY",
    "SlippageZ",
    "IsAttenuating",
    "SoundScale",
    "SoundValues",
    "SoundLevelTimeSeries",
    "Frequency",
    "SoundLevelSingleValue",
    "ApplicableValueRatio",
    "ThermalLoadSource",
    "PropertySource",
    "SourceDescription",
    "MaximumValue",
    "MinimumValue",
    "ThermalLoadTimeSeriesValues",
    "UserDefinedThermalLoadSource",
    "UserDefinedPropertySource",
    "ThermalLoadType",
    "LinearForceX",
    "LinearForceY",
    "LinearForceZ",
    "LinearMomentX",
    "LinearMomentY",
    "LinearMomentZ",
    "PlanarForceX",
    "PlanarForceY",
    "PlanarForceZ",
    "DisplacementX",
    "DisplacementY",
    "DisplacementZ",
    "RotationalDisplacementRX",
    "RotationalDisplacementRY",
    "RotationalDisplacementRZ",
    "Distortion",
    "ForceX",
    "ForceY",
    "ForceZ",
    "MomentX",
    "MomentY",
    "MomentZ",
    "WarpingMoment",
    "TorsionalConstantX",
    "MomentOfInertiaYZ",
    "MomentOfInertiaY",
    "MomentOfInertiaZ",
    "WarpingConstant",
    "ShearCentreZ",
    "ShearCentreY",
    "ShearDeformationAreaZ",
    "ShearDeformationAreaY",
    "MaximumSectionModulusY",
    "MinimumSectionModulusY",
    "MaximumSectionModulusZ",
    "MinimumSectionModulusZ",
    "TorsionalSectionModulus",
    "CentreOfGravityInX",
    "CentreOfGravityInY",
    "ShearAreaZ",
    "ShearAreaY",
    "PlasticShapeFactorY",
    "PlasticShapeFactorZ",
    "ParentEdge",
    "Transparency",
    "DiffuseColour",
    "ReflectionColour",
    "SpecularColour",
    "SpecularHighlight",
    "ReflectanceMethod",
    "SweptArea",
    "Directrix",
    "InnerRadius",
    "StartParam",
    "EndParam",
    "SweptCurve",
    "Depth",
    "FlangeWidth",
    "WebThickness",
    "FlangeThickness",
    "FilletRadius",
    "FlangeEdgeRadius",
    "WebEdgeRadius",
    "WebSlope",
    "FlangeSlope",
    "AnnotatedCurve",
    "Literal",
    "Placement",
    "Path",
    "Extent",
    "BoxAlignment",
    "BottomXDim",
    "TopXDim",
    "TopXOffset",
    "SecondRepeatFactor",
    "ApplicableOccurrence",
    "HasPropertySets",
    "RepresentationMaps",
    "Tag",
    "EdgeRadius",
    "Magnitude",
    "LoopVertex",
    "LiningDepth",
    "LiningThickness",
    "TransomThickness",
    "MullionThickness",
    "FirstTransomOffset",
    "SecondTransomOffset",
    "FirstMullionOffset",
    "SecondMullionOffset",
    "ShapeAspectStyle",
    "OperationType",
    "PanelPosition",
    "FrameDepth",
    "FrameThickness",
    "ConstructionType",
    "ParameterTakesPrecedence",
    "Sizeable",
    "OuterBoundary",
    "InnerBoundaries",
    "FillStyleTarget",
    "GlobalOrLocal",
    "TextureCoordinates",
    "Axis",
    "RefDirection",
    "FirstOperand",
    "SecondOperand",
    "Corner",
    "ZDim",
    "Enclosure",
    "WallThickness",
    "Girth",
    "InternalFilletRadius",
    "Axis1",
    "Axis2",
    "LocalOrigin",
    "Scale",
    "Scale2",
    "Axis3",
    "Scale3",
    "Transition",
    "ParentCurve",
    "OverallHeight",
    "BaseWidth2",
    "HeadWidth",
    "HeadDepth2",
    "HeadDepth3",
    "BaseWidth4",
    "BaseDepth1",
    "BaseDepth2",
    "BaseDepth3",
    "TreeRootExpression",
    "Definition",
    "Target",
    "DirectionRatios",
    "ThresholdDepth",
    "ThresholdThickness",
    "TransomOffset",
    "LiningOffset",
    "ThresholdOffset",
    "CasingThickness",
    "CasingDepth",
    "PanelDepth",
    "PanelOperation",
    "PanelWidth",
    "Contents",
    "MethodOfMeasurement",
    "Quantities",
    "ElementType",
    "SemiAxis1",
    "SemiAxis2",
    "EnergySequence",
    "UserDefinedEnergySequence",
    "ExtrudedDirection",
    "FbsmFaces",
    "HatchLineAppearance",
    "StartOfNextHatchLine",
    "PointOfReferenceHatchLine",
    "PatternStart",
    "HatchLineAngle",
    "Symbol",
    "TilingPattern",
    "Tiles",
    "TilingScale",
    "FlowConditionTimeSeries",
    "VelocityTimeSeries",
    "FlowrateTimeSeries",
    "Fluid",
    "PressureTimeSeries",
    "TemperatureSingleValue",
    "WetBulbTemperatureSingleValue",
    "WetBulbTemperatureTimeSeries",
    "TemperatureTimeSeries",
    "FlowrateSingleValue",
    "FlowConditionSingleValue",
    "VelocitySingleValue",
    "PressureSingleValue",
    "AssemblyPlace",
    "OverallWidth",
    "OverallDepth",
    "LegSlope",
    "Pnt",
    "Dir",
    "Outer",
    "ObjectType",
    "Distance",
    "SelfIntersect",
    "ObjectPlacement",
    "Representation",
    "LongName",
    "Phase",
    "RepresentationContexts",
    "UnitsInContext",
    "ProxyType",
    "InnerFilletRadius",
    "OuterFilletRadius",
    "XLength",
    "YLength",
    "U1",
    "V1",
    "U2",
    "V2",
    "Usense",
    "Vsense",
    "RelatedObjects",
    "RelatedObjectsType",
    "RelatingActor",
    "ActingRole",
    "RelatingControl",
    "RelatingGroup",
    "RelatingProcess",
    "QuantityInProcess",
    "RelatingProduct",
    "RelatingResource",
    "RelatingAppliedValue",
    "RelatingClassification",
    "Intent",
    "RelatingLibrary",
    "RelatingMaterial",
    "RelatingProfileProperties",
    "ProfileSectionLocation",
    "ProfileOrientation",
    "ConnectionGeometry",
    "RelatingElement",
    "RelatedElement",
    "RelatingPriorities",
    "RelatedPriorities",
    "RelatedConnectionType",
    "RelatingConnectionType",
    "RelatingPort",
    "RelatedPort",
    "RealizingElement",
    "RelatedStructuralActivity",
    "RelatedStructuralMember",
    "RelatingStructuralMember",
    "RelatedStructuralConnection",
    "AppliedCondition",
    "AdditionalConditions",
    "SupportedLength",
    "ConditionCoordinateSystem",
    "ConnectionConstraint",
    "RealizingElements",
    "ConnectionType",
    "RelatedElements",
    "RelatingStructure",
    "RelatingBuildingElement",
    "RelatedCoverings",
    "RelatedSpace",
    "RelatingObject",
    "RelatingPropertyDefinition",
    "RelatingType",
    "RelatingOpeningElement",
    "RelatedBuildingElement",
    "RelatedControlElements",
    "RelatingFlowElement",
    "DailyInteraction",
    "ImportanceRating",
    "LocationOfInteraction",
    "RelatedSpaceProgram",
    "RelatingSpaceProgram",
    "OverridingProperties",
    "RelatedFeatureElement",
    "RelatedProcess",
    "TimeLag",
    "SequenceType",
    "RelatingSystem",
    "RelatedBuildings",
    "RelatingSpace",
    "PhysicalOrVirtualBoundary",
    "InternalOrExternalBoundary",
    "RelatedOpeningElement",
    "Angle",
    "BottomRadius",
    "CompositionType",
    "AppliedLoad",
    "SubsequentThickness",
    "VaryingThicknessLocation",
    "ReferenceSurface",
    "AxisPosition",
    "TaskId",
    "WorkMethod",
    "IsMilestone",
    "Priority",
    "TheActor",
    "TopFlangeWidth",
    "TopFlangeThickness",
    "TopFlangeFilletRadius",
    "ZLength",
    "ElevationOfRefHeight",
    "ElevationOfTerrain",
    "BuildingAddress",
    "Elevation",
    "Segments",
    "ResourceIdentifier",
    "ResourceGroup",
    "ResourceConsumption",
    "BaseQuantity",
    "SubmittedBy",
    "PreparedBy",
    "SubmittedOn",
    "TargetUsers",
    "UpdateDate",
    "ID",
    "ElectricCurrentType",
    "InputVoltage",
    "InputFrequency",
    "FullLoadCurrent",
    "MinimumCircuitCurrent",
    "MaximumPowerInput",
    "RatedPowerInput",
    "InputPhase",
    "Voids",
    "UAxes",
    "VAxes",
    "WAxes",
    "InventoryType",
    "Jurisdiction",
    "ResponsiblePersons",
    "LastUpdateDate",
    "CurrentValue",
    "OriginalValue",
    "SkillSet",
    "NominalDiameter",
    "NominalLength",
    "MoveFrom",
    "MoveTo",
    "PunchList",
    "ActionID",
    "LifeCyclePhase",
    "PermitID",
    "Points",
    "ProcedureID",
    "ProcedureType",
    "UserDefinedProcedureType",
    "Records",
    "TimeForTask",
    "ActualStart",
    "EarlyStart",
    "LateStart",
    "ScheduleStart",
    "ActualFinish",
    "EarlyFinish",
    "LateFinish",
    "ScheduleFinish",
    "ScheduleDuration",
    "ActualDuration",
    "RemainingTime",
    "FreeFloat",
    "TotalFloat",
    "IsCritical",
    "StatusTime",
    "StartFloat",
    "FinishFloat",
    "Completion",
    "ServiceLifeType",
    "ServiceLifeDuration",
    "RefLatitude",
    "RefLongitude",
    "RefElevation",
    "LandTitleNumber",
    "SiteAddress",
    "InteriorOrExteriorSpace",
    "ElevationWithFlooring",
    "SpaceProgramIdentifier",
    "MaxRequiredArea",
    "MinRequiredArea",
    "RequestedLocation",
    "StandardRequiredArea",
    "DestabilizingLoad",
    "CausedBy",
    "ProjectedOrTrue",
    "VaryingAppliedLoadLocation",
    "SubsequentAppliedLoads",
    "ActionType",
    "ActionSource",
    "Coefficient",
    "TheoryType",
    "ResultForLoadGroup",
    "IsLinear",
    "SubContractor",
    "JobDescription",
    "ApplicableDates",
    "TimeSeriesScheduleType",
    "TimeSeries",
    "CapacityByWeight",
    "CapacityByNumber",
    "Trim1",
    "Trim2",
    "SenseAgreement",
    "MasterRepresentation",
    "Creators",
    "Duration",
    "FinishTime",
    "WorkControlType",
    "UserDefinedControlType",
    "RequestID",
    "AssetID",
    "TotalReplacementCost",
    "Owner",
    "User",
    "ResponsiblePerson",
    "IncorporationDate",
    "DepreciatedValue",
    "Degree",
    "ControlPointsList",
    "CurveForm",
    "ClosedCurve",
    "Criterion",
    "CriterionDateTime",
    "Suppliers",
    "UsageRatio",
    "FlowDirection",
    "FeatureLength",
    "ShapeType",
    "WeightsData",
    "MeshLength",
    "MeshWidth",
    "LongitudinalBarNominalDiameter",
    "TransverseBarNominalDiameter",
    "LongitudinalBarCrossSectionArea",
    "TransverseBarCrossSectionArea",
    "LongitudinalBarSpacing",
    "TransverseBarSpacing",
    "NumberOfRiser",
    "NumberOfTreads",
    "RiserHeight",
    "TreadLength",
    "OrientationOf2DPlane",
    "LoadedBy",
    "HasResults",
    "TensionForce",
    "PreStress",
    "FrictionCoefficient",
    "AnchorageSlip",
    "MinCurvatureRadius",
    "ControlElementId",
    "DistributionPointFunction",
    "UserDefinedFunction",
    "BarLength",
    "BarRole",
    "TimeOfApproval",
    "Level",
    "Qualifier",
    "RequestingApproval",
    "GivingApproval",
    "TranslationalStiffnessByLengthX",
    "TranslationalStiffnessByLengthY",
    "TranslationalStiffnessByLengthZ",
    "TranslationalStiffnessByAreaX",
    "TranslationalStiffnessByAreaY",
    "TranslationalStiffnessByAreaZ",
    "TranslationalStiffnessX",
    "TranslationalStiffnessY",
    "TranslationalStiffnessZ",
    "VolumeOnRelatingElement",
    "VolumeOnRelatedElement",
    "SourceCRS",
    "TargetCRS",
    "GeodeticDatum",
    "VerticalDatum",
    "Identification",
    "Language",
    "ReferencedLibrary",
    "Eastings",
    "Northings",
    "OrthogonalHeight",
    "XAxisAbscissa",
    "XAxisOrdinate",
    "OffsetDirection",
    "OffsetValues",
    "Profile",
    "MaterialProfiles",
    "CompositeProfile",
    "ReferencePath",
    "MapProjection",
    "MapZone",
    "MapUnit",
    "Formula",
    "RecurrenceType",
    "WeekdayComponent",
    "Interval",
    "Occurrences",
    "TimePeriods",
    "TypeIdentifier",
    "AttributeIdentifier",
    "InstanceName",
    "ListPositions",
    "InnerReference",
    "Locations",
    "DeltaTConstant",
    "DeltaTY",
    "DeltaTZ",
    "SurfaceReinforcement1",
    "SurfaceReinforcement2",
    "ShearReinforcement",
    "Columns",
    "DurationType",
    "Recurrence",
    "MessagingIDs",
    "ModelOrDraughting",
    "Maps",
    "Vertices",
    "MappedTo",
    "TexCoordsList",
    "RecurrencePattern",
    "Start",
    "Finish",
    "RelatedApprovals",
    "ReferenceTokens",
    "Sort",
    "ColourList",
    "ConversionOffset",
    "ActualDate",
    "EarlyDate",
    "LateDate",
    "ScheduleDate",
    "Properties",
    "RelatingReference",
    "RelatedResourceObjects",
    "ModelorDraughting",
    "URLReference",
    "Opacity",
    "Colours",
    "ColourIndex",
    "TexCoords",
    "TexCoordIndex",
    "LagValue",
    "Fraction",
    "MaterialConstituents",
    "ReferenceExtent",
    "ForProfileSet",
    "CardinalPoint",
    "ForProfileEndSet",
    "CardinalEndPoint",
    "RelatedMaterials",
    "ScheduleWork",
    "ScheduleUsage",
    "ScheduleContour",
    "LevelingDelay",
    "IsOverAllocated",
    "ActualWork",
    "ActualUsage",
    "RemainingWork",
    "RemainingUsage",
    "LongDescription",
    "ProcessType",
    "ResourceType",
    "BottomFlangeWidth",
    "BottomFlangeThickness",
    "BottomFlangeFilletRadius",
    "BottomFlangeEdgeRadius",
    "BottomFlangeSlope",
    "TopFlangeEdgeRadius",
    "TopFlangeSlope",
    "CoordList",
    "BaseCosts",
    "Boundaries",
    "ImplicitOuter",
    "EventTriggerType",
    "UserDefinedEventTriggerType",
    "EndSweptArea",
    "FixedReference",
    "CoordIndex",
    "InnerCoordIndices",
    "ReferenceCurve",
    "SetPointValue",
    "TemplateType",
    "ApplicableEntity",
    "HasPropertyTemplates",
    "CurveInterpolation",
    "Factor",
    "RelatingContext",
    "RelatedDefinitions",
    "RelatedPropertySets",
    "RelatingTemplate",
    "InterferenceGeometry",
    "InterferenceType",
    "ImpliedOrder",
    "UserDefinedSequenceType",
    "ParentBoundary",
    "CorrespondingBoundary",
    "ParamLength",
    "PrimaryMeasureType",
    "SecondaryMeasureType",
    "Enumerators",
    "PrimaryUnit",
    "SecondaryUnit",
    "AccessState",
    "Curve3D",
    "AssociatedGeometry",
    "TaskTime",
    "MajorRadius",
    "MinorRadius",
    "Normals",
    "Closed",
    "PnIndex",
    "LiningToPanelOffsetX",
    "LiningToPanelOffsetY",
    "UDegree",
    "VDegree",
    "SurfaceForm",
    "UClosed",
    "VClosed",
    "UMultiplicities",
    "VMultiplicities",
    "UKnots",
    "VKnots",
    "KnotSpec",
    "CostValues",
    "CostQuantities",
    "UserDefinedOperationType",
    "EventOccurenceTime",
    "Faces",
    "BendingShapeCode",
    "BendingParameters",
    "SheathDiameter",
    "PartitioningType",
    "UserDefinedPartitioningType",
    "WorkingTimes",
    "ExceptionTimes",
    "KnotMultiplicities",
    "Knots",
    "SystemType",
    "NumberOfRisers",
    "SharedPlacement",
    "SelfWeightCoefficients",
    "StartTag",
    "EndTag",
    "StartDistAlong",
    "HorizontalLength",
    "StartHeight",
    "StartGradient",
    "EndGradient",
    "RadiusOfCurvature",
    "PrimeMeridian",
    "AngleUnit",
    "HeightUnit",
    "FactorX",
    "FactorY",
    "FactorZ",
    "NumberValue",
    "FirstCoordinate",
    "SecondCoordinate",
    "TexCoordsOf",
    "InnerTexCoordIndices",
    "WellKnownText",
    "CoordinateReferenceSystem",
    "StartDate",
    "FinishDate",
    "StartCantLeft",
    "EndCantLeft",
    "StartCantRight",
    "EndCantRight",
    "StartPoint",
    "StartDirection",
    "StartRadiusOfCurvature",
    "EndRadiusOfCurvature",
    "SegmentLength",
    "GravityCenterLineHeight",
    "Specification",
    "CurveStyleFont",
    "CartesianPosition",
    "MaterialExpression",
    "HorizontalWidths",
    "Widths",
    "Slopes",
    "Tags",
    "OffsetPoint",
    "DistanceAlong",
    "OffsetLateral",
    "OffsetVertical",
    "OffsetLongitudinal",
    "TagList",
    "SegmentStart",
    "TexCoordIndices",
    "CoefficientsX",
    "CoefficientsY",
    "CoefficientsZ",
    "RelatingProfileDef",
    "InterferenceSpace",
    "RelatingPositioningElement",
    "RelatedProducts",
    "CubicTerm",
    "QuadraticTerm",
    "LinearTerm",
    "ConstantTerm",
    "Flags",
    "ClothoidConstant",
    "CosineTerm",
    "UsageType",
    "BaseCurve",
    "EndPoint",
    "RelatedSurfaceFeatures",
    "SepticTerm",
    "SexticTerm",
    "QuinticTerm",
    "QuarticTerm",
    "SineTerm",
    "AxisDirection",
    "RailHeadDistance",
    "DesignParameters",
];

pub const PROP_TYPE_NAMES: [u32; 617] = [
    3869224543,
    3258342251,
    2801250643,
    33568735,
    4251960020,
    983778844,
    2887218128,
    2597039031,
    2679630077,
    411424972,
    373436428,
    1728812236,
    130549933,
    3630933823,
    2598011224,
    2173214787,
    1052454078,
    1753493141,
    1307019551,
    3211557302,
    1718600412,
    86635668,
    765770214,
    4065007721,
    622194075,
    3639012971,
    747523909,
    1767535486,
    3931646380,
    382301979,
    3958567839,
    2844211000,
    4111266820,
    1959218052,
    2449831054,
    2680653174,
    2589826445,
    102610177,
    1641986514,
    2706619895,
    1245737093,
    1072939445,
    2655187982,
    3510044353,
    3345948710,
    1243674935,
    2815919920,
    30780891,
    1045800335,
    124742581,
    1918398963,
    1718147282,
    3732053477,
    1376555844,
    2885466731,
    4068098364,
    1154170062,
    3073041342,
    3376698491,
    2601014836,
    2735952531,
    3521532855,
    3452421091,
    4042175685,
    2755797622,
    881902783,
    4162380809,
    2766185779,
    1065062679,
    300323983,
    1838606355,
    503418787,
    248100487,
    3303938423,
    1469346588,
    3389681023,
    1103567559,
    69416015,
    3341486342,
    2281867870,
    3665567075,
    1222501353,
    207745069,
    1042063629,
    2917043736,
    2949456006,
    1197507443,
    3368373690,
    1962769620,
    618182010,
    101040310,
    639542469,
    4223916898,
    531202833,
    2591213694,
    2077209135,
    939592812,
    148024130,
    287783114,
    1076942058,
    3477203348,
    2321227483,
    2650437152,
    1778710042,
    2726807636,
    3458127941,
    3124614049,
    3247369562,
    1190328964,
    2095195183,
    3377609919,
    3008791417,
    274646400,
    3064340077,
    1207048766,
    1787361927,
    542029231,
    2342653256,
    3202202375,
    2042790032,
    1580146022,
    3982875396,
    673634403,
    743184107,
    2417041796,
    2785408664,
    1225378771,
    776857604,
    200335297,
    626085974,
    3049289330,
    1381336441,
    3749851601,
    2597065344,
    1850111279,
    531007025,
    916597516,
    2562834741,
    1286164555,
    603696268,
    1102727119,
    2715512545,
    2590844177,
    1570177309,
    3052078743,
    1460886941,
    3490877962,
    296282323,
    1844851602,
    3304826586,
    2260317790,
    2645777649,
    1432008316,
    290688911,
    3101149627,
    1210645708,
    1123145078,
    2067069095,
    852622518,
    3686016028,
    929793134,
    1260505505,
    2556980723,
    4149869811,
    3733406562,
    2799835756,
    1809719519,
    1008929658,
    2513912981,
    1361398929,
    544876936,
    1158859006,
    1648970520,
    1477762836,
    3531705166,
    4134073009,
    32440307,
    3448662350,
    1131349010,
    3346224455,
    891718957,
    3192672207,
    3345633955,
    3177669450,
    3020489413,
    2740243338,
    2095003142,
    3034186359,
    3739410009,
    3054510233,
    3701648758,
    1660063152,
    59481748,
    1417489154,
    3900360178,
    3125803723,
    1029017970,
    2483315170,
    1939436016,
    3710013099,
    1430971844,
    581633288,
    4165799628,
    3732776249,
    387828814,
    990564147,
    2298722686,
    3657550814,
    1202362311,
    3044325142,
    1050256046,
    2895544493,
    506871491,
    1364037233,
    85334491,
    191860431,
    2128979029,
    2642773653,
    94842927,
    1278329552,
    3114022597,
    51269191,
    3467162246,
    1877383524,
    3921983062,
    1011845978,
    3288037868,
    2169031380,
    1152197495,
    1663979128,
    1867003952,
    3357820518,
    867548509,
    2324037503,
    278839091,
    3672713367,
    1074166056,
    3637616042,
    2453401579,
    280115917,
    3054888242,
    1532845080,
    2581212453,
    1914407012,
    717039860,
    220341763,
    2460950869,
    3481340091,
    3571493279,
    2872136011,
    509816776,
    2721224556,
    2504768628,
    1025434211,
    370225590,
    3800577675,
    2356011799,
    3612888222,
    2833995503,
    4250110687,
    3086160713,
    1925676203,
    2205249479,
    3446698506,
    2095639259,
    180925521,
    1545711075,
    219451334,
    2296667514,
    3293443760,
    2706460486,
    2945172077,
    4208778838,
    2914609552,
    2341007311,
    288382656,
    4274534246,
    2802850158,
    2898209111,
    2859738748,
    1758889154,
    2128902557,
    3740093272,
    2688182192,
    3544373492,
    530289379,
    1179482911,
    4037036970,
    2273995522,
    2706606064,
    1973544240,
    3856911033,
    3888040117,
    1628702193,
    3588315303,
    1062813311,
    3040386961,
    652456506,
    2143335405,
    628493158,
    2254336722,
    64643665,
    1687521235,
    1287392070,
    4261334040,
    690167070,
    2162789131,
    3337205297,
    3740788744,
    3355820592,
    1460979143,
    2485617015,
    3924139846,
    672692152,
    2966862399,
    2451242878,
    1021431103,
    2506197118,
    3790457270,
    2841622424,
    1002142388,
    554647353,
    2973211341,
    1154284921,
    1007984134,
    237118112,
    2774431236,
    602808272,
    1844818999,
    185388416,
    4215032627,
    3818625751,
    527936033,
    3349296550,
    103775553,
    3531860660,
    2293803863,
    661370862,
    3037870609,
    1551283683,
    3372526763,
    386187035,
    3739419792,
    473029300,
    1491040762,
    1040890966,
    3517283431,
    1367202144,
    2934217365,
    3812528620,
    639531123,
    1855850635,
    1455546828,
    3956248403,
    3038022802,
    3689010777,
    1094947699,
    960326014,
    2602792976,
    2261624226,
    126693432,
    96294661,
    2506162743,
    1252848954,
    1942645678,
    3300536621,
    4121373105,
    291444547,
    437759802,
    3407053508,
    3627328112,
    935604799,
    3594581223,
    3551551017,
    3676660675,
    3340908731,
    1269596434,
    1239913253,
    1797193231,
    3573632694,
    358033588,
    3726661758,
    1913101020,
    54623293,
    1501183454,
    649472068,
    1693487766,
    1760651496,
    798148481,
    3453182476,
    3089591714,
    3405941096,
    668377315,
    2079224331,
    8322439,
    2183683140,
    376935608,
    1280103771,
    1875623387,
    1290156191,
    3044747827,
    3185663589,
    3531917241,
    667340609,
    1262424489,
    3754373064,
    3959380518,
    2793383123,
    897523405,
    4155216521,
    2974343352,
    2457772935,
    3995464546,
    1736192930,
    1098295817,
    2319738306,
    3881097202,
    2986769608,
    3917635812,
    1282226622,
    815500815,
    4164688622,
    3805913727,
    3470481846,
    937566702,
    2195413836,
    613796396,
    4241973650,
    1880189351,
    972054012,
    2509546566,
    854899952,
    11730523,
    2853304871,
    1466758467,
    950732822,
    1275358634,
    49088397,
    525895558,
    2235152071,
    1485152156,
    2433181523,
    3119450353,
    1390679141,
    3701338814,
    1199560280,
    4021806647,
    609421318,
    3172978893,
    2043862942,
    3555794193,
    2541165894,
    3915482550,
    2636378356,
    1640371178,
    4075327185,
    2314439260,
    3714063296,
    3200245327,
    4036359239,
    440562759,
    2387106220,
    3285139300,
    1790229001,
    3611470254,
    3242977126,
    3708119000,
    164193824,
    1683019596,
    760658860,
    2952703181,
    1136057603,
    1401066283,
    3717035687,
    3958052878,
    1639589134,
    460077198,
    1162880614,
    606860825,
    3521284610,
    3547450287,
    1959371038,
    2211051443,
    2207572250,
    736530666,
    1945004755,
    1412071761,
    3419103109,
    49845113,
    492091185,
    1585845231,
    4134219045,
    3523091289,
    1521410863,
    3841475323,
    3647622174,
    316539858,
    2860242611,
    4008630002,
    1682466193,
    960210175,
    1021971458,
    1549132990,
    384449397,
    2059837836,
    37940459,
    2053683727,
    627898853,
    1692979113,
    91683625,
    3593671318,
    506783830,
    1042787934,
    683809370,
    1970628803,
    4016286979,
    3462168616,
    211053100,
    2244117335,
    3041753155,
    574549367,
    183626358,
    2548949139,
    962935207,
    3989067775,
    2874063949,
    571176181,
    3061959087,
    178912537,
    1897649832,
    1385270127,
    2739565819,
    1823282114,
    244819378,
    3876018962,
    2680421541,
    652748602,
    1479426229,
    3288126668,
    255461614,
    1932549289,
    1236880293,
    1471118587,
    2706281606,
    3098684301,
    4151168619,
    2981638260,
    3476419373,
    3114819794,
    1973315761,
    2089642407,
    614319689,
    697765865,
    860830233,
    922449830,
    365584592,
    3344706444,
    2749697471,
    145283476,
    3301026240,
    1505327130,
    2395907400,
    2149462589,
    2969962241,
    3194911961,
    3425423356,
    3166912612,
    2004835150,
    222769930,
    2165702409,
    1805707277,
    1837433645,
    463610769,
    1946335990,
    2394031724,
    1184275752,
    3733744356,
    823603102,
    1420568751,
    2447993252,
    1019252178,
    3178974365,
    4013007887,
    2615076639,
    3099164984,
    4135496989,
    1095732595,
    423474865,
    2707447046,
    3294834125,
    1938929368,
    2906317437,
    827741273,
    2181869104,
    1718859833,
    3101698114,
    1464019863,
    3629595153,
    3160627042,
    1506544127,
    2641080392,
    859079163,
    1672225696,
    2630368378,
    2879124712,
    3124462625,
    1536983066,
    492794765,
    1338660958,
    4218053802,
    1906401893,
    3829999316,
    1268632640,
    2872680054,
    1128263546,
    2875026444,
    2326367582,
];

#[inline]
pub fn get_property_type_code(schema: IFC_SCHEMA, type_code: u32, prop: u32) -> u32 {
    match schema {
        IFC_SCHEMA::IFC2X3 => match type_code {
            0 => match prop {
                _ => 0,
            },
            1 => match prop {
                _ => 0,
            },
            2 => match prop {
                _ => 0,
            },
            3 => match prop {
                _ => 0,
            },
            4 => match prop {
                _ => 0,
            },
            5 => match prop {
                _ => 0,
            },
            6 => match prop {
                _ => 0,
            },
            7 => match prop {
                _ => 0,
            },
            8 => match prop {
                _ => 0,
            },
            9 => match prop {
                _ => 0,
            },
            10 => match prop {
                _ => 0,
            },
            11 => match prop {
                _ => 0,
            },
            12 => match prop {
                _ => 0,
            },
            13 => match prop {
                _ => 0,
            },
            14 => match prop {
                _ => 0,
            },
            15 => match prop {
                _ => 0,
            },
            16 => match prop {
                _ => 0,
            },
            17 => match prop {
                _ => 0,
            },
            18 => match prop {
                _ => 0,
            },
            19 => match prop {
                _ => 0,
            },
            20 => match prop {
                _ => 0,
            },
            21 => match prop {
                _ => 0,
            },
            22 => match prop {
                _ => 0,
            },
            23 => match prop {
                _ => 0,
            },
            24 => match prop {
                _ => 0,
            },
            25 => match prop {
                _ => 0,
            },
            26 => match prop {
                _ => 0,
            },
            5716631 => match prop {
                _ => 0,
            },
            30780891 => match prop {
                _ => 0,
            },
            32440307 => match prop {
                _ => 0,
            },
            45288368 => match prop {
                _ => 0,
            },
            52481810 => match prop {
                _ => 0,
            },
            59481748 => match prop {
                _ => 0,
            },
            80994333 => match prop {
                _ => 0,
            },
            101040310 => match prop {
                _ => 0,
            },
            103090709 => match prop {
                _ => 0,
            },
            110355661 => match prop {
                _ => 0,
            },
            125510826 => match prop {
                _ => 0,
            },
            130549933 => match prop {
                _ => 0,
            },
            148013059 => match prop {
                _ => 0,
            },
            148025276 => match prop {
                _ => 0,
            },
            160246688 => match prop {
                _ => 0,
            },
            178086475 => match prop {
                _ => 0,
            },
            179317114 => match prop {
                _ => 0,
            },
            180925521 => match prop {
                _ => 0,
            },
            194851669 => match prop {
                _ => 0,
            },
            200128114 => match prop {
                _ => 0,
            },
            202636808 => match prop {
                _ => 0,
            },
            205026976 => match prop {
                _ => 0,
            },
            214636428 => match prop {
                _ => 0,
            },
            219451334 => match prop {
                _ => 0,
            },
            220341763 => match prop {
                _ => 0,
            },
            230924584 => match prop {
                _ => 0,
            },
            231477066 => match prop {
                _ => 0,
            },
            248100487 => match prop {
                _ => 0,
            },
            263784265 => match prop {
                _ => 0,
            },
            279856033 => match prop {
                _ => 0,
            },
            280115917 => match prop {
                _ => 0,
            },
            300633059 => match prop {
                _ => 0,
            },
            315944413 => match prop {
                _ => 0,
            },
            331165859 => match prop {
                _ => 0,
            },
            335055490 => match prop {
                _ => 0,
            },
            336235671 => match prop {
                _ => 0,
            },
            339256511 => match prop {
                _ => 0,
            },
            346874300 => match prop {
                _ => 0,
            },
            347226245 => match prop {
                _ => 0,
            },
            360485395 => match prop {
                _ => 0,
            },
            366585022 => match prop {
                _ => 0,
            },
            370225590 => match prop {
                _ => 0,
            },
            374418227 => match prop {
                _ => 0,
            },
            377706215 => match prop {
                _ => 0,
            },
            390701378 => match prop {
                _ => 0,
            },
            390851274 => match prop {
                _ => 0,
            },
            395041908 => match prop {
                _ => 0,
            },
            395920057 => match prop {
                _ => 0,
            },
            411424972 => match prop {
                _ => 0,
            },
            427810014 => match prop {
                _ => 0,
            },
            433424934 => match prop {
                _ => 0,
            },
            445594917 => match prop {
                _ => 0,
            },
            448429030 => match prop {
                _ => 0,
            },
            451544542 => match prop {
                _ => 0,
            },
            476780140 => match prop {
                _ => 0,
            },
            477187591 => match prop {
                _ => 0,
            },
            478536968 => match prop {
                _ => 0,
            },
            488727124 => match prop {
                _ => 0,
            },
            504942748 => match prop {
                _ => 0,
            },
            512836454 => match prop {
                _ => 0,
            },
            526551008 => match prop {
                _ => 0,
            },
            530289379 => match prop {
                _ => 0,
            },
            531007025 => match prop {
                _ => 0,
            },
            539742890 => match prop {
                _ => 0,
            },
            572779678 => match prop {
                _ => 0,
            },
            578613899 => match prop {
                _ => 0,
            },
            581633288 => match prop {
                _ => 0,
            },
            597895409 => match prop {
                _ => 0,
            },
            602808272 => match prop {
                _ => 0,
            },
            603570806 => match prop {
                _ => 0,
            },
            606661476 => match prop {
                _ => 0,
            },
            613356794 => match prop {
                _ => 0,
            },
            616511568 => match prop {
                _ => 0,
            },
            618182010 => match prop {
                _ => 0,
            },
            622194075 => match prop {
                _ => 0,
            },
            626085974 => match prop {
                _ => 0,
            },
            639542469 => match prop {
                _ => 0,
            },
            647756555 => match prop {
                _ => 0,
            },
            647927063 => match prop {
                _ => 0,
            },
            652456506 => match prop {
                _ => 0,
            },
            663422040 => match prop {
                _ => 0,
            },
            669184980 => match prop {
                _ => 0,
            },
            673634403 => match prop {
                _ => 0,
            },
            677618848 => match prop {
                _ => 0,
            },
            681481545 => match prop {
                _ => 0,
            },
            682877961 => match prop {
                _ => 0,
            },
            693640335 => match prop {
                _ => 0,
            },
            693772133 => match prop {
                _ => 0,
            },
            707683696 => match prop {
                _ => 0,
            },
            712377611 => match prop {
                _ => 0,
            },
            723233188 => match prop {
                _ => 0,
            },
            728799441 => match prop {
                _ => 0,
            },
            734778138 => match prop {
                _ => 0,
            },
            738692330 => match prop {
                _ => 0,
            },
            747523909 => match prop {
                _ => 0,
            },
            750771296 => match prop {
                _ => 0,
            },
            753842376 => match prop {
                _ => 0,
            },
            759155922 => match prop {
                _ => 0,
            },
            770865208 => match prop {
                _ => 0,
            },
            776857604 => match prop {
                _ => 0,
            },
            781010003 => match prop {
                _ => 0,
            },
            803316827 => match prop {
                _ => 0,
            },
            803998398 => match prop {
                _ => 0,
            },
            804291784 => match prop {
                _ => 0,
            },
            807026263 => match prop {
                _ => 0,
            },
            812098782 => match prop {
                _ => 0,
            },
            814719939 => match prop {
                _ => 0,
            },
            819618141 => match prop {
                _ => 0,
            },
            825690147 => match prop {
                _ => 0,
            },
            826625072 => match prop {
                _ => 0,
            },
            843113511 => match prop {
                _ => 0,
            },
            846575682 => match prop {
                _ => 0,
            },
            852622518 => match prop {
                _ => 0,
            },
            855621170 => match prop {
                _ => 0,
            },
            857184966 => match prop {
                _ => 0,
            },
            867548509 => match prop {
                _ => 0,
            },
            869906466 => match prop {
                _ => 0,
            },
            871118103 => match prop {
                _ => 0,
            },
            886880790 => match prop {
                _ => 0,
            },
            891718957 => match prop {
                _ => 0,
            },
            900683007 => match prop {
                _ => 0,
            },
            912023232 => match prop {
                _ => 0,
            },
            919958153 => match prop {
                _ => 0,
            },
            931644368 => match prop {
                _ => 0,
            },
            938368621 => match prop {
                _ => 0,
            },
            941946838 => match prop {
                _ => 0,
            },
            962685235 => match prop {
                _ => 0,
            },
            977012517 => match prop {
                _ => 0,
            },
            979691226 => match prop {
                _ => 0,
            },
            982818633 => match prop {
                _ => 0,
            },
            985171141 => match prop {
                _ => 0,
            },
            987401354 => match prop {
                _ => 0,
            },
            987898635 => match prop {
                _ => 0,
            },
            990879717 => match prop {
                _ => 0,
            },
            1008929658 => match prop {
                _ => 0,
            },
            1028945134 => match prop {
                _ => 0,
            },
            1029017970 => match prop {
                _ => 0,
            },
            1033361043 => match prop {
                _ => 0,
            },
            1039846685 => match prop {
                _ => 0,
            },
            1040185647 => match prop {
                _ => 0,
            },
            1045800335 => match prop {
                _ => 0,
            },
            1051575348 => match prop {
                _ => 0,
            },
            1052013943 => match prop {
                _ => 0,
            },
            1058617721 => match prop {
                _ => 0,
            },
            1060000209 => match prop {
                _ => 0,
            },
            1062813311 => match prop {
                _ => 0,
            },
            1065062679 => match prop {
                _ => 0,
            },
            1065908215 => match prop {
                _ => 0,
            },
            1072939445 => match prop {
                _ => 0,
            },
            1073191201 => match prop {
                _ => 0,
            },
            1076942058 => match prop {
                _ => 0,
            },
            1095909175 => match prop {
                _ => 0,
            },
            1098599126 => match prop {
                _ => 0,
            },
            1105321065 => match prop {
                _ => 0,
            },
            1110488051 => match prop {
                _ => 0,
            },
            1123145078 => match prop {
                _ => 0,
            },
            1133259667 => match prop {
                _ => 0,
            },
            1154170062 => match prop {
                _ => 0,
            },
            1161773419 => match prop {
                _ => 0,
            },
            1163958913 => match prop {
                _ => 0,
            },
            1179482911 => match prop {
                _ => 0,
            },
            1190533807 => match prop {
                _ => 0,
            },
            1202362311 => match prop {
                _ => 0,
            },
            1204542856 => match prop {
                _ => 0,
            },
            1207048766 => match prop {
                _ => 0,
            },
            1210645708 => match prop {
                _ => 0,
            },
            1213861670 => match prop {
                _ => 0,
            },
            1217240411 => match prop {
                _ => 0,
            },
            1222501353 => match prop {
                _ => 0,
            },
            1227763645 => match prop {
                _ => 0,
            },
            1235345126 => match prop {
                _ => 0,
            },
            1245217292 => match prop {
                _ => 0,
            },
            1251058090 => match prop {
                _ => 0,
            },
            1252848954 => match prop {
                _ => 0,
            },
            1260505505 => match prop {
                _ => 0,
            },
            1260650574 => match prop {
                _ => 0,
            },
            1268542332 => match prop {
                _ => 0,
            },
            1281925730 => match prop {
                _ => 0,
            },
            1285652485 => match prop {
                _ => 0,
            },
            1287392070 => match prop {
                _ => 0,
            },
            1290481447 => match prop {
                _ => 0,
            },
            1299126871 => match prop {
                _ => 0,
            },
            1300840506 => match prop {
                _ => 0,
            },
            1302238472 => match prop {
                _ => 0,
            },
            1303795690 => match prop {
                _ => 0,
            },
            1304840413 => match prop {
                _ => 0,
            },
            1305183839 => match prop {
                _ => 0,
            },
            1307041759 => match prop {
                _ => 0,
            },
            1310608509 => match prop {
                _ => 0,
            },
            1327628568 => match prop {
                _ => 0,
            },
            1334484129 => match prop {
                _ => 0,
            },
            1335981549 => match prop {
                _ => 0,
            },
            1339347760 => match prop {
                _ => 0,
            },
            1345879162 => match prop {
                _ => 0,
            },
            1351298697 => match prop {
                _ => 0,
            },
            1365060375 => match prop {
                _ => 0,
            },
            1376555844 => match prop {
                _ => 0,
            },
            1376911519 => match prop {
                _ => 0,
            },
            1377556343 => match prop {
                _ => 0,
            },
            1383045692 => match prop {
                _ => 0,
            },
            1387855156 => match prop {
                _ => 0,
            },
            1401173127 => match prop {
                _ => 0,
            },
            1402838566 => match prop {
                _ => 0,
            },
            1411181986 => match prop {
                _ => 0,
            },
            1411407467 => match prop {
                _ => 0,
            },
            1416205885 => match prop {
                _ => 0,
            },
            1417489154 => match prop {
                _ => 0,
            },
            1419761937 => match prop {
                _ => 0,
            },
            1423911732 => match prop {
                _ => 0,
            },
            1425443689 => match prop {
                _ => 0,
            },
            1430189142 => match prop {
                _ => 0,
            },
            1446786286 => match prop {
                _ => 0,
            },
            1447204868 => match prop {
                _ => 0,
            },
            1451395588 => match prop {
                _ => 0,
            },
            1457835157 => match prop {
                _ => 0,
            },
            1472233963 => match prop {
                _ => 0,
            },
            1482959167 => match prop {
                _ => 0,
            },
            1484403080 => match prop {
                _ => 0,
            },
            1484833681 => match prop {
                _ => 0,
            },
            1485152156 => match prop {
                _ => 0,
            },
            1509187699 => match prop {
                _ => 0,
            },
            1520743889 => match prop {
                _ => 0,
            },
            1529196076 => match prop {
                _ => 0,
            },
            1534661035 => match prop {
                _ => 0,
            },
            1560379544 => match prop {
                _ => 0,
            },
            1566485204 => match prop {
                _ => 0,
            },
            1580146022 => match prop {
                _ => 0,
            },
            1580310250 => match prop {
                _ => 0,
            },
            1595516126 => match prop {
                _ => 0,
            },
            1597423693 => match prop {
                _ => 0,
            },
            1599208980 => match prop {
                _ => 0,
            },
            1600972822 => match prop {
                _ => 0,
            },
            1607154358 => match prop {
                _ => 0,
            },
            1620046519 => match prop {
                _ => 0,
            },
            1621171031 => match prop {
                _ => 0,
            },
            1623761950 => match prop {
                _ => 0,
            },
            1628702193 => match prop {
                _ => 0,
            },
            1634875225 => match prop {
                _ => 0,
            },
            1637806684 => match prop {
                _ => 0,
            },
            1638771189 => match prop {
                _ => 0,
            },
            1640371178 => match prop {
                _ => 0,
            },
            1648886627 => match prop {
                _ => 0,
            },
            1658513725 => match prop {
                _ => 0,
            },
            1658829314 => match prop {
                _ => 0,
            },
            1660063152 => match prop {
                _ => 0,
            },
            1663979128 => match prop {
                _ => 0,
            },
            1674181508 => match prop {
                _ => 0,
            },
            1680319473 => match prop {
                _ => 0,
            },
            1683148259 => match prop {
                _ => 0,
            },
            1687234759 => match prop {
                _ => 0,
            },
            1692211062 => match prop {
                _ => 0,
            },
            1694125774 => match prop {
                _ => 0,
            },
            1704287377 => match prop {
                _ => 0,
            },
            1714330368 => match prop {
                _ => 0,
            },
            1718945513 => match prop {
                _ => 0,
            },
            1721250024 => match prop {
                _ => 0,
            },
            1735638870 => match prop {
                _ => 0,
            },
            1742049831 => match prop {
                _ => 0,
            },
            1758889154 => match prop {
                _ => 0,
            },
            1765591967 => match prop {
                _ => 0,
            },
            1767535486 => match prop {
                _ => 0,
            },
            1768891740 => match prop {
                _ => 0,
            },
            1775413392 => match prop {
                _ => 0,
            },
            1783015770 => match prop {
                _ => 0,
            },
            1806887404 => match prop {
                _ => 0,
            },
            1807405624 => match prop {
                _ => 0,
            },
            1809719519 => match prop {
                _ => 0,
            },
            1810631287 => match prop {
                _ => 0,
            },
            1834744321 => match prop {
                _ => 0,
            },
            1838606355 => match prop {
                _ => 0,
            },
            1842657554 => match prop {
                _ => 0,
            },
            1847130766 => match prop {
                _ => 0,
            },
            1856042241 => match prop {
                _ => 0,
            },
            1860660968 => match prop {
                _ => 0,
            },
            1865459582 => match prop {
                _ => 0,
            },
            1871374353 => match prop {
                _ => 0,
            },
            1878645084 => match prop {
                _ => 0,
            },
            1883228015 => match prop {
                _ => 0,
            },
            1898987631 => match prop {
                _ => 0,
            },
            1907098498 => match prop {
                _ => 0,
            },
            1909888760 => match prop {
                _ => 0,
            },
            1911125066 => match prop {
                _ => 0,
            },
            1916426348 => match prop {
                _ => 0,
            },
            1916936684 => match prop {
                _ => 0,
            },
            1916977116 => match prop {
                _ => 0,
            },
            1918398963 => match prop {
                _ => 0,
            },
            1945004755 => match prop {
                _ => 0,
            },
            1950629157 => match prop {
                _ => 0,
            },
            1959218052 => match prop {
                _ => 0,
            },
            1962604670 => match prop {
                _ => 0,
            },
            1967976161 => match prop {
                _ => 0,
            },
            1973038258 => match prop {
                _ => 0,
            },
            1973544240 => match prop {
                _ => 0,
            },
            1975003073 => match prop {
                _ => 0,
            },
            1981873012 => match prop {
                _ => 0,
            },
            1983826977 => match prop {
                _ => 0,
            },
            2004835150 => match prop {
                _ => 0,
            },
            2016517767 => match prop {
                _ => 0,
            },
            2022407955 => match prop {
                _ => 0,
            },
            2022622350 => match prop {
                _ => 0,
            },
            2028607225 => match prop {
                _ => 0,
            },
            2030761528 => match prop {
                _ => 0,
            },
            2042790032 => match prop {
                _ => 0,
            },
            2044713172 => match prop {
                _ => 0,
            },
            2047409740 => match prop {
                _ => 0,
            },
            2051452291 => match prop {
                _ => 0,
            },
            2058353004 => match prop {
                _ => 0,
            },
            2063403501 => match prop {
                _ => 0,
            },
            2067069095 => match prop {
                _ => 0,
            },
            2069777674 => match prop {
                _ => 0,
            },
            2077209135 => match prop {
                _ => 0,
            },
            2080292479 => match prop {
                _ => 0,
            },
            2082059205 => match prop {
                _ => 0,
            },
            2093928680 => match prop {
                _ => 0,
            },
            2095639259 => match prop {
                _ => 0,
            },
            2097647324 => match prop {
                _ => 0,
            },
            2107101300 => match prop {
                _ => 0,
            },
            2108223431 => match prop {
                _ => 0,
            },
            2127690289 => match prop {
                _ => 0,
            },
            2143335405 => match prop {
                _ => 0,
            },
            2147822146 => match prop {
                _ => 0,
            },
            2162789131 => match prop {
                _ => 0,
            },
            2188551683 => match prop {
                _ => 0,
            },
            2199411900 => match prop {
                _ => 0,
            },
            2205249479 => match prop {
                _ => 0,
            },
            2218152070 => match prop {
                _ => 0,
            },
            2223149337 => match prop {
                _ => 0,
            },
            2226359599 => match prop {
                _ => 0,
            },
            2233826070 => match prop {
                _ => 0,
            },
            2242383968 => match prop {
                _ => 0,
            },
            2247615214 => match prop {
                _ => 0,
            },
            2250791053 => match prop {
                _ => 0,
            },
            2251480897 => match prop {
                _ => 0,
            },
            2254336722 => match prop {
                _ => 0,
            },
            2262370178 => match prop {
                _ => 0,
            },
            2265737646 => match prop {
                _ => 0,
            },
            2267347899 => match prop {
                _ => 0,
            },
            2273265877 => match prop {
                _ => 0,
            },
            2273995522 => match prop {
                _ => 0,
            },
            2296667514 => match prop {
                _ => 0,
            },
            2297155007 => match prop {
                _ => 0,
            },
            2297822566 => match prop {
                _ => 0,
            },
            2301859152 => match prop {
                _ => 0,
            },
            2315554128 => match prop {
                _ => 0,
            },
            2320036040 => match prop {
                _ => 0,
            },
            2324767716 => match prop {
                _ => 0,
            },
            2341007311 => match prop {
                _ => 0,
            },
            2347385850 => match prop {
                _ => 0,
            },
            2347447852 => match prop {
                _ => 0,
            },
            2347495698 => match prop {
                _ => 0,
            },
            2367409068 => match prop {
                _ => 0,
            },
            2382730787 => match prop {
                _ => 0,
            },
            2391368822 => match prop {
                _ => 0,
            },
            2391406946 => match prop {
                _ => 0,
            },
            2405470396 => match prop {
                _ => 0,
            },
            2411513650 => match prop {
                _ => 0,
            },
            2417041796 => match prop {
                _ => 0,
            },
            2442683028 => match prop {
                _ => 0,
            },
            2445078500 => match prop {
                _ => 0,
            },
            2445595289 => match prop {
                _ => 0,
            },
            2453401579 => match prop {
                _ => 0,
            },
            2454782716 => match prop {
                _ => 0,
            },
            2470393545 => match prop {
                _ => 0,
            },
            2473145415 => match prop {
                _ => 0,
            },
            2483315170 => match prop {
                _ => 0,
            },
            2485617015 => match prop {
                _ => 0,
            },
            2485662743 => match prop {
                _ => 0,
            },
            2489546625 => match prop {
                _ => 0,
            },
            2495723537 => match prop {
                _ => 0,
            },
            2506170314 => match prop {
                _ => 0,
            },
            2506943328 => match prop {
                _ => 0,
            },
            2510884976 => match prop {
                _ => 0,
            },
            2513912981 => match prop {
                _ => 0,
            },
            2515109513 => match prop {
                _ => 0,
            },
            2519244187 => match prop {
                _ => 0,
            },
            2525727697 => match prop {
                _ => 0,
            },
            2529465313 => match prop {
                _ => 0,
            },
            2533589738 => match prop {
                _ => 0,
            },
            2542286263 => match prop {
                _ => 0,
            },
            2543172580 => match prop {
                _ => 0,
            },
            2551354335 => match prop {
                _ => 0,
            },
            2552916305 => match prop {
                _ => 0,
            },
            2556980723 => match prop {
                _ => 0,
            },
            2559016684 => match prop {
                _ => 0,
            },
            2559216714 => match prop {
                _ => 0,
            },
            2581212453 => match prop {
                _ => 0,
            },
            2590856083 => match prop {
                _ => 0,
            },
            2597039031 => match prop {
                _ => 0,
            },
            2598011224 => match prop {
                _ => 0,
            },
            2601014836 => match prop {
                _ => 0,
            },
            2604431987 => match prop {
                _ => 0,
            },
            2609359061 => match prop {
                _ => 0,
            },
            2611217952 => match prop {
                _ => 0,
            },
            2614616156 => match prop {
                _ => 0,
            },
            2624227202 => match prop {
                _ => 0,
            },
            2635815018 => match prop {
                _ => 0,
            },
            2636378356 => match prop {
                _ => 0,
            },
            2655187982 => match prop {
                _ => 0,
            },
            2655215786 => match prop {
                _ => 0,
            },
            2665983363 => match prop {
                _ => 0,
            },
            2668620305 => match prop {
                _ => 0,
            },
            2692823254 => match prop {
                _ => 0,
            },
            2705031697 => match prop {
                _ => 0,
            },
            2706460486 => match prop {
                _ => 0,
            },
            2706606064 => match prop {
                _ => 0,
            },
            2706619895 => match prop {
                _ => 0,
            },
            2713105998 => match prop {
                _ => 0,
            },
            2715220739 => match prop {
                _ => 0,
            },
            2728634034 => match prop {
                _ => 0,
            },
            2732653382 => match prop {
                _ => 0,
            },
            2736907675 => match prop {
                _ => 0,
            },
            2740243338 => match prop {
                _ => 0,
            },
            2744685151 => match prop {
                _ => 0,
            },
            2752243245 => match prop {
                _ => 0,
            },
            2759199220 => match prop {
                _ => 0,
            },
            2769231204 => match prop {
                _ => 0,
            },
            2770003689 => match prop {
                _ => 0,
            },
            2775532180 => match prop {
                _ => 0,
            },
            2777663545 => match prop {
                _ => 0,
            },
            2778083089 => match prop {
                _ => 0,
            },
            2798486643 => match prop {
                _ => 0,
            },
            2799835756 => match prop {
                _ => 0,
            },
            2802773753 => match prop {
                _ => 0,
            },
            2802850158 => match prop {
                _ => 0,
            },
            2809605785 => match prop {
                _ => 0,
            },
            2816379211 => match prop {
                _ => 0,
            },
            2827207264 => match prop {
                _ => 0,
            },
            2827736869 => match prop {
                _ => 0,
            },
            2830218821 => match prop {
                _ => 0,
            },
            2833995503 => match prop {
                _ => 0,
            },
            2835456948 => match prop {
                _ => 0,
            },
            2837617999 => match prop {
                _ => 0,
            },
            2851387026 => match prop {
                _ => 0,
            },
            2857406711 => match prop {
                _ => 0,
            },
            2859738748 => match prop {
                _ => 0,
            },
            2863920197 => match prop {
                _ => 0,
            },
            2874132201 => match prop {
                _ => 0,
            },
            2889183280 => match prop {
                _ => 0,
            },
            2893384427 => match prop {
                _ => 0,
            },
            2898889636 => match prop {
                _ => 0,
            },
            2904328755 => match prop {
                _ => 0,
            },
            2914609552 => match prop {
                _ => 0,
            },
            2924175390 => match prop {
                _ => 0,
            },
            2937912522 => match prop {
                _ => 0,
            },
            2945172077 => match prop {
                _ => 0,
            },
            2949456006 => match prop {
                _ => 0,
            },
            2951183804 => match prop {
                _ => 0,
            },
            2954562838 => match prop {
                _ => 0,
            },
            2963535650 => match prop {
                _ => 0,
            },
            2979338954 => match prop {
                _ => 0,
            },
            2986769608 => match prop {
                _ => 0,
            },
            3001207471 => match prop {
                _ => 0,
            },
            3008276851 => match prop {
                _ => 0,
            },
            3008791417 => match prop {
                _ => 0,
            },
            3009204131 => match prop {
                _ => 0,
            },
            3009222698 => match prop {
                _ => 0,
            },
            3020489413 => match prop {
                _ => 0,
            },
            3021840470 => match prop {
                _ => 0,
            },
            3024970846 => match prop {
                _ => 0,
            },
            3027567501 => match prop {
                _ => 0,
            },
            3028897424 => match prop {
                _ => 0,
            },
            3040386961 => match prop {
                _ => 0,
            },
            3041715199 => match prop {
                _ => 0,
            },
            3049322572 => match prop {
                _ => 0,
            },
            3050246964 => match prop {
                _ => 0,
            },
            3055160366 => match prop {
                _ => 0,
            },
            3071757647 => match prop {
                _ => 0,
            },
            3073041342 => match prop {
                _ => 0,
            },
            3101149627 => match prop {
                _ => 0,
            },
            3112655638 => match prop {
                _ => 0,
            },
            3119450353 => match prop {
                _ => 0,
            },
            3124254112 => match prop {
                _ => 0,
            },
            3124975700 => match prop {
                _ => 0,
            },
            3125803723 => match prop {
                _ => 0,
            },
            3132237377 => match prop {
                _ => 0,
            },
            3136571912 => match prop {
                _ => 0,
            },
            3150382593 => match prop {
                _ => 0,
            },
            3171933400 => match prop {
                _ => 0,
            },
            3174744832 => match prop {
                _ => 0,
            },
            3181161470 => match prop {
                _ => 0,
            },
            3190031847 => match prop {
                _ => 0,
            },
            3198132628 => match prop {
                _ => 0,
            },
            3200245327 => match prop {
                _ => 0,
            },
            3207319532 => match prop {
                _ => 0,
            },
            3207858831 => match prop {
                _ => 0,
            },
            3213052703 => match prop {
                _ => 0,
            },
            3219374653 => match prop {
                _ => 0,
            },
            3242617779 => match prop {
                _ => 0,
            },
            3248260540 => match prop {
                _ => 0,
            },
            3252649465 => match prop {
                _ => 0,
            },
            3256556792 => match prop {
                _ => 0,
            },
            3264961684 => match prop {
                _ => 0,
            },
            3265635763 => match prop {
                _ => 0,
            },
            3268803585 => match prop {
                _ => 0,
            },
            3272907226 => match prop {
                _ => 0,
            },
            3277789161 => match prop {
                _ => 0,
            },
            3283111854 => match prop {
                _ => 0,
            },
            3288037868 => match prop {
                _ => 0,
            },
            3293443760 => match prop {
                _ => 0,
            },
            3293546465 => match prop {
                _ => 0,
            },
            3295246426 => match prop {
                _ => 0,
            },
            3299480353 => match prop {
                _ => 0,
            },
            3303107099 => match prop {
                _ => 0,
            },
            3303938423 => match prop {
                _ => 0,
            },
            3304561284 => match prop {
                _ => 0,
            },
            3304826586 => match prop {
                _ => 0,
            },
            3313531582 => match prop {
                _ => 0,
            },
            3317419933 => match prop {
                _ => 0,
            },
            3327091369 => match prop {
                _ => 0,
            },
            3331915920 => match prop {
                _ => 0,
            },
            3342526732 => match prop {
                _ => 0,
            },
            3352864051 => match prop {
                _ => 0,
            },
            3355820592 => match prop {
                _ => 0,
            },
            3357820518 => match prop {
                _ => 0,
            },
            3367102660 => match prop {
                _ => 0,
            },
            3368373690 => match prop {
                _ => 0,
            },
            3372526763 => match prop {
                _ => 0,
            },
            3377609919 => match prop {
                _ => 0,
            },
            3388369263 => match prop {
                _ => 0,
            },
            3390157468 => match prop {
                _ => 0,
            },
            3408363356 => match prop {
                _ => 0,
            },
            3413951693 => match prop {
                _ => 0,
            },
            3422422726 => match prop {
                _ => 0,
            },
            3425660407 => match prop {
                _ => 0,
            },
            3448662350 => match prop {
                _ => 0,
            },
            3451746338 => match prop {
                _ => 0,
            },
            3452421091 => match prop {
                _ => 0,
            },
            3454111270 => match prop {
                _ => 0,
            },
            3455213021 => match prop {
                _ => 0,
            },
            3460190687 => match prop {
                _ => 0,
            },
            3473067441 => match prop {
                _ => 0,
            },
            3486308946 => match prop {
                _ => 0,
            },
            3495092785 => match prop {
                _ => 0,
            },
            3505215534 => match prop {
                _ => 0,
            },
            3508470533 => match prop {
                _ => 0,
            },
            3510044353 => match prop {
                _ => 0,
            },
            3512223829 => match prop {
                _ => 0,
            },
            3517283431 => match prop {
                _ => 0,
            },
            3544373492 => match prop {
                _ => 0,
            },
            3548104201 => match prop {
                _ => 0,
            },
            3566463478 => match prop {
                _ => 0,
            },
            3588315303 => match prop {
                _ => 0,
            },
            3590301190 => match prop {
                _ => 0,
            },
            3593883385 => match prop {
                _ => 0,
            },
            3612888222 => match prop {
                _ => 0,
            },
            3615266464 => match prop {
                _ => 0,
            },
            3626867408 => match prop {
                _ => 0,
            },
            3630933823 => match prop {
                _ => 0,
            },
            3632507154 => match prop {
                _ => 0,
            },
            3639012971 => match prop {
                _ => 0,
            },
            3642467123 => match prop {
                _ => 0,
            },
            3649129432 => match prop {
                _ => 0,
            },
            3650150729 => match prop {
                _ => 0,
            },
            3651124850 => match prop {
                _ => 0,
            },
            3653947884 => match prop {
                _ => 0,
            },
            3678494232 => match prop {
                _ => 0,
            },
            3679540991 => match prop {
                _ => 0,
            },
            3689010777 => match prop {
                _ => 0,
            },
            3692461612 => match prop {
                _ => 0,
            },
            3700593921 => match prop {
                _ => 0,
            },
            3701648758 => match prop {
                _ => 0,
            },
            3710013099 => match prop {
                _ => 0,
            },
            3724593414 => match prop {
                _ => 0,
            },
            3727388367 => match prop {
                _ => 0,
            },
            3732053477 => match prop {
                _ => 0,
            },
            3732776249 => match prop {
                _ => 0,
            },
            3737207727 => match prop {
                _ => 0,
            },
            3740093272 => match prop {
                _ => 0,
            },
            3741457305 => match prop {
                _ => 0,
            },
            3749851601 => match prop {
                _ => 0,
            },
            3760055223 => match prop {
                _ => 0,
            },
            3765753017 => match prop {
                _ => 0,
            },
            3796139169 => match prop {
                _ => 0,
            },
            3798115385 => match prop {
                _ => 0,
            },
            3800577675 => match prop {
                _ => 0,
            },
            3812236995 => match prop {
                _ => 0,
            },
            3815607619 => match prop {
                _ => 0,
            },
            3821786052 => match prop {
                _ => 0,
            },
            3824725483 => match prop {
                _ => 0,
            },
            3827777499 => match prop {
                _ => 0,
            },
            3840914261 => match prop {
                _ => 0,
            },
            3843319758 => match prop {
                _ => 0,
            },
            3849074793 => match prop {
                _ => 0,
            },
            3850581409 => match prop {
                _ => 0,
            },
            3856911033 => match prop {
                _ => 0,
            },
            3857492461 => match prop {
                _ => 0,
            },
            3869604511 => match prop {
                _ => 0,
            },
            3888040117 => match prop {
                _ => 0,
            },
            3893378262 => match prop {
                _ => 0,
            },
            3895139033 => match prop {
                _ => 0,
            },
            3896028662 => match prop {
                _ => 0,
            },
            3898045240 => match prop {
                _ => 0,
            },
            3900360178 => match prop {
                _ => 0,
            },
            3905492369 => match prop {
                _ => 0,
            },
            3907093117 => match prop {
                _ => 0,
            },
            3912681535 => match prop {
                _ => 0,
            },
            3939117080 => match prop {
                _ => 0,
            },
            3940055652 => match prop {
                _ => 0,
            },
            3945020480 => match prop {
                _ => 0,
            },
            3958052878 => match prop {
                _ => 0,
            },
            3958567839 => match prop {
                _ => 0,
            },
            3961806047 => match prop {
                _ => 0,
            },
            3979015343 => match prop {
                _ => 0,
            },
            3982875396 => match prop {
                _ => 0,
            },
            3987759626 => match prop {
                _ => 0,
            },
            4006246654 => match prop {
                _ => 0,
            },
            4017108033 => match prop {
                _ => 0,
            },
            4022376103 => match prop {
                _ => 0,
            },
            4031249490 => match prop {
                _ => 0,
            },
            4037036970 => match prop {
                _ => 0,
            },
            4037862832 => match prop {
                _ => 0,
            },
            4054601972 => match prop {
                _ => 0,
            },
            4070609034 => match prop {
                _ => 0,
            },
            4095574036 => match prop {
                _ => 0,
            },
            4097777520 => match prop {
                _ => 0,
            },
            4105383287 => match prop {
                _ => 0,
            },
            4122056220 => match prop {
                _ => 0,
            },
            4123344466 => match prop {
                _ => 0,
            },
            4124623270 => match prop {
                _ => 0,
            },
            4124788165 => match prop {
                _ => 0,
            },
            4133800736 => match prop {
                _ => 0,
            },
            4142052618 => match prop {
                _ => 0,
            },
            4143007308 => match prop {
                _ => 0,
            },
            4147604152 => match prop {
                _ => 0,
            },
            4158566097 => match prop {
                _ => 0,
            },
            4162380809 => match prop {
                _ => 0,
            },
            4165799628 => match prop {
                _ => 0,
            },
            4166981789 => match prop {
                _ => 0,
            },
            4170525392 => match prop {
                _ => 0,
            },
            4182860854 => match prop {
                _ => 0,
            },
            4186316022 => match prop {
                _ => 0,
            },
            4189434867 => match prop {
                _ => 0,
            },
            4194566429 => match prop {
                _ => 0,
            },
            4201705270 => match prop {
                _ => 0,
            },
            4203026998 => match prop {
                _ => 0,
            },
            4208778838 => match prop {
                _ => 0,
            },
            4218914973 => match prop {
                _ => 0,
            },
            4219587988 => match prop {
                _ => 0,
            },
            4222183408 => match prop {
                _ => 0,
            },
            4231323485 => match prop {
                _ => 0,
            },
            4238390223 => match prop {
                _ => 0,
            },
            4240577450 => match prop {
                _ => 0,
            },
            4243806635 => match prop {
                _ => 0,
            },
            4251960020 => match prop {
                _ => 0,
            },
            4252922144 => match prop {
                _ => 0,
            },
            4256014907 => match prop {
                _ => 0,
            },
            4257277454 => match prop {
                _ => 0,
            },
            4261334040 => match prop {
                _ => 0,
            },
            4266656042 => match prop {
                _ => 0,
            },
            4278684876 => match prop {
                _ => 0,
            },
            4278956645 => match prop {
                _ => 0,
            },
            4282788508 => match prop {
                _ => 0,
            },
            4288270099 => match prop {
                _ => 0,
            },
            _ => 0,
        },
        IFC_SCHEMA::IFC4 => match type_code {
            0 => match prop {
                _ => 0,
            },
            1 => match prop {
                _ => 0,
            },
            2 => match prop {
                _ => 0,
            },
            3 => match prop {
                _ => 0,
            },
            4 => match prop {
                _ => 0,
            },
            5 => match prop {
                _ => 0,
            },
            6 => match prop {
                _ => 0,
            },
            7 => match prop {
                _ => 0,
            },
            8 => match prop {
                _ => 0,
            },
            9 => match prop {
                _ => 0,
            },
            10 => match prop {
                _ => 0,
            },
            11 => match prop {
                _ => 0,
            },
            12 => match prop {
                _ => 0,
            },
            13 => match prop {
                _ => 0,
            },
            14 => match prop {
                _ => 0,
            },
            15 => match prop {
                _ => 0,
            },
            16 => match prop {
                _ => 0,
            },
            17 => match prop {
                _ => 0,
            },
            18 => match prop {
                _ => 0,
            },
            19 => match prop {
                _ => 0,
            },
            20 => match prop {
                _ => 0,
            },
            5716631 => match prop {
                _ => 0,
            },
            15328376 => match prop {
                _ => 0,
            },
            25142252 => match prop {
                _ => 0,
            },
            32344328 => match prop {
                _ => 0,
            },
            32440307 => match prop {
                _ => 0,
            },
            39481116 => match prop {
                _ => 0,
            },
            45288368 => match prop {
                _ => 0,
            },
            59481748 => match prop {
                _ => 0,
            },
            76236018 => match prop {
                _ => 0,
            },
            90941305 => match prop {
                _ => 0,
            },
            101040310 => match prop {
                _ => 0,
            },
            103090709 => match prop {
                _ => 0,
            },
            110355661 => match prop {
                _ => 0,
            },
            125510826 => match prop {
                _ => 0,
            },
            130549933 => match prop {
                _ => 0,
            },
            132023988 => match prop {
                _ => 0,
            },
            144952367 => match prop {
                _ => 0,
            },
            148013059 => match prop {
                _ => 0,
            },
            148025276 => match prop {
                _ => 0,
            },
            160246688 => match prop {
                _ => 0,
            },
            164193824 => match prop {
                _ => 0,
            },
            167062518 => match prop {
                _ => 0,
            },
            177149247 => match prop {
                _ => 0,
            },
            178086475 => match prop {
                _ => 0,
            },
            178912537 => match prop {
                _ => 0,
            },
            180925521 => match prop {
                _ => 0,
            },
            182646315 => match prop {
                _ => 0,
            },
            205026976 => match prop {
                _ => 0,
            },
            211053100 => match prop {
                _ => 0,
            },
            214636428 => match prop {
                _ => 0,
            },
            219451334 => match prop {
                _ => 0,
            },
            220341763 => match prop {
                _ => 0,
            },
            230924584 => match prop {
                _ => 0,
            },
            231477066 => match prop {
                _ => 0,
            },
            248100487 => match prop {
                _ => 0,
            },
            263784265 => match prop {
                _ => 0,
            },
            264262732 => match prop {
                _ => 0,
            },
            277319702 => match prop {
                _ => 0,
            },
            279856033 => match prop {
                _ => 0,
            },
            280115917 => match prop {
                _ => 0,
            },
            297599258 => match prop {
                _ => 0,
            },
            300633059 => match prop {
                _ => 0,
            },
            307848117 => match prop {
                _ => 0,
            },
            310824031 => match prop {
                _ => 0,
            },
            315944413 => match prop {
                _ => 0,
            },
            331165859 => match prop {
                _ => 0,
            },
            335055490 => match prop {
                _ => 0,
            },
            336235671 => match prop {
                _ => 0,
            },
            338393293 => match prop {
                _ => 0,
            },
            339256511 => match prop {
                _ => 0,
            },
            342316401 => match prop {
                _ => 0,
            },
            346874300 => match prop {
                _ => 0,
            },
            366585022 => match prop {
                _ => 0,
            },
            370225590 => match prop {
                _ => 0,
            },
            374418227 => match prop {
                _ => 0,
            },
            377706215 => match prop {
                _ => 0,
            },
            385403989 => match prop {
                _ => 0,
            },
            395041908 => match prop {
                _ => 0,
            },
            395920057 => match prop {
                _ => 0,
            },
            400855858 => match prop {
                _ => 0,
            },
            402227799 => match prop {
                _ => 0,
            },
            411424972 => match prop {
                _ => 0,
            },
            413509423 => match prop {
                _ => 0,
            },
            427810014 => match prop {
                _ => 0,
            },
            427948657 => match prop {
                _ => 0,
            },
            428585644 => match prop {
                _ => 0,
            },
            445594917 => match prop {
                _ => 0,
            },
            448429030 => match prop {
                _ => 0,
            },
            451544542 => match prop {
                _ => 0,
            },
            463610769 => match prop {
                _ => 0,
            },
            476780140 => match prop {
                _ => 0,
            },
            477187591 => match prop {
                _ => 0,
            },
            478536968 => match prop {
                _ => 0,
            },
            484807127 => match prop {
                _ => 0,
            },
            486154966 => match prop {
                _ => 0,
            },
            488727124 => match prop {
                _ => 0,
            },
            492091185 => match prop {
                _ => 0,
            },
            504942748 => match prop {
                _ => 0,
            },
            512836454 => match prop {
                _ => 0,
            },
            526551008 => match prop {
                _ => 0,
            },
            530289379 => match prop {
                _ => 0,
            },
            531007025 => match prop {
                _ => 0,
            },
            539742890 => match prop {
                _ => 0,
            },
            552965576 => match prop {
                _ => 0,
            },
            562808652 => match prop {
                _ => 0,
            },
            569719735 => match prop {
                _ => 0,
            },
            572779678 => match prop {
                _ => 0,
            },
            574549367 => match prop {
                _ => 0,
            },
            578613899 => match prop {
                _ => 0,
            },
            581633288 => match prop {
                _ => 0,
            },
            597895409 => match prop {
                _ => 0,
            },
            602808272 => match prop {
                _ => 0,
            },
            603570806 => match prop {
                _ => 0,
            },
            603775116 => match prop {
                _ => 0,
            },
            609421318 => match prop {
                _ => 0,
            },
            616511568 => match prop {
                _ => 0,
            },
            618182010 => match prop {
                _ => 0,
            },
            626085974 => match prop {
                _ => 0,
            },
            629592764 => match prop {
                _ => 0,
            },
            630975310 => match prop {
                _ => 0,
            },
            635142910 => match prop {
                _ => 0,
            },
            639361253 => match prop {
                _ => 0,
            },
            639542469 => match prop {
                _ => 0,
            },
            647756555 => match prop {
                _ => 0,
            },
            647927063 => match prop {
                _ => 0,
            },
            653396225 => match prop {
                _ => 0,
            },
            655969474 => match prop {
                _ => 0,
            },
            663422040 => match prop {
                _ => 0,
            },
            669184980 => match prop {
                _ => 0,
            },
            673634403 => match prop {
                _ => 0,
            },
            677532197 => match prop {
                _ => 0,
            },
            682877961 => match prop {
                _ => 0,
            },
            683857671 => match prop {
                _ => 0,
            },
            693640335 => match prop {
                _ => 0,
            },
            699246055 => match prop {
                _ => 0,
            },
            707683696 => match prop {
                _ => 0,
            },
            710998568 => match prop {
                _ => 0,
            },
            712377611 => match prop {
                _ => 0,
            },
            723233188 => match prop {
                _ => 0,
            },
            728799441 => match prop {
                _ => 0,
            },
            734778138 => match prop {
                _ => 0,
            },
            738039164 => match prop {
                _ => 0,
            },
            738692330 => match prop {
                _ => 0,
            },
            747523909 => match prop {
                _ => 0,
            },
            750771296 => match prop {
                _ => 0,
            },
            753842376 => match prop {
                _ => 0,
            },
            759155922 => match prop {
                _ => 0,
            },
            760658860 => match prop {
                _ => 0,
            },
            770865208 => match prop {
                _ => 0,
            },
            775493141 => match prop {
                _ => 0,
            },
            776857604 => match prop {
                _ => 0,
            },
            781010003 => match prop {
                _ => 0,
            },
            803316827 => match prop {
                _ => 0,
            },
            804291784 => match prop {
                _ => 0,
            },
            807026263 => match prop {
                _ => 0,
            },
            812098782 => match prop {
                _ => 0,
            },
            812556717 => match prop {
                _ => 0,
            },
            816062949 => match prop {
                _ => 0,
            },
            819412036 => match prop {
                _ => 0,
            },
            819618141 => match prop {
                _ => 0,
            },
            825690147 => match prop {
                _ => 0,
            },
            826625072 => match prop {
                _ => 0,
            },
            843113511 => match prop {
                _ => 0,
            },
            846575682 => match prop {
                _ => 0,
            },
            852622518 => match prop {
                _ => 0,
            },
            853536259 => match prop {
                _ => 0,
            },
            862014818 => match prop {
                _ => 0,
            },
            867548509 => match prop {
                _ => 0,
            },
            869906466 => match prop {
                _ => 0,
            },
            871118103 => match prop {
                _ => 0,
            },
            886880790 => match prop {
                _ => 0,
            },
            891718957 => match prop {
                _ => 0,
            },
            900683007 => match prop {
                _ => 0,
            },
            901063453 => match prop {
                _ => 0,
            },
            905975707 => match prop {
                _ => 0,
            },
            912023232 => match prop {
                _ => 0,
            },
            919958153 => match prop {
                _ => 0,
            },
            926996030 => match prop {
                _ => 0,
            },
            931644368 => match prop {
                _ => 0,
            },
            941946838 => match prop {
                _ => 0,
            },
            964333572 => match prop {
                _ => 0,
            },
            977012517 => match prop {
                _ => 0,
            },
            979691226 => match prop {
                _ => 0,
            },
            982818633 => match prop {
                _ => 0,
            },
            985171141 => match prop {
                _ => 0,
            },
            986844984 => match prop {
                _ => 0,
            },
            987401354 => match prop {
                _ => 0,
            },
            987898635 => match prop {
                _ => 0,
            },
            1003880860 => match prop {
                _ => 0,
            },
            1004757350 => match prop {
                _ => 0,
            },
            1008929658 => match prop {
                _ => 0,
            },
            1027710054 => match prop {
                _ => 0,
            },
            1028945134 => match prop {
                _ => 0,
            },
            1029017970 => match prop {
                _ => 0,
            },
            1033361043 => match prop {
                _ => 0,
            },
            1039846685 => match prop {
                _ => 0,
            },
            1040185647 => match prop {
                _ => 0,
            },
            1042787934 => match prop {
                _ => 0,
            },
            1045800335 => match prop {
                _ => 0,
            },
            1051575348 => match prop {
                _ => 0,
            },
            1051757585 => match prop {
                _ => 0,
            },
            1052013943 => match prop {
                _ => 0,
            },
            1054537805 => match prop {
                _ => 0,
            },
            1060000209 => match prop {
                _ => 0,
            },
            1062813311 => match prop {
                _ => 0,
            },
            1072016465 => match prop {
                _ => 0,
            },
            1073191201 => match prop {
                _ => 0,
            },
            1076942058 => match prop {
                _ => 0,
            },
            1095909175 => match prop {
                _ => 0,
            },
            1096409881 => match prop {
                _ => 0,
            },
            1105321065 => match prop {
                _ => 0,
            },
            1114901282 => match prop {
                _ => 0,
            },
            1123145078 => match prop {
                _ => 0,
            },
            1133259667 => match prop {
                _ => 0,
            },
            1136057603 => match prop {
                _ => 0,
            },
            1154170062 => match prop {
                _ => 0,
            },
            1156407060 => match prop {
                _ => 0,
            },
            1158309216 => match prop {
                _ => 0,
            },
            1161773419 => match prop {
                _ => 0,
            },
            1162798199 => match prop {
                _ => 0,
            },
            1177604601 => match prop {
                _ => 0,
            },
            1179482911 => match prop {
                _ => 0,
            },
            1190533807 => match prop {
                _ => 0,
            },
            1199560280 => match prop {
                _ => 0,
            },
            1204542856 => match prop {
                _ => 0,
            },
            1207048766 => match prop {
                _ => 0,
            },
            1209101575 => match prop {
                _ => 0,
            },
            1210645708 => match prop {
                _ => 0,
            },
            1213902940 => match prop {
                _ => 0,
            },
            1217240411 => match prop {
                _ => 0,
            },
            1232101972 => match prop {
                _ => 0,
            },
            1235345126 => match prop {
                _ => 0,
            },
            1236880293 => match prop {
                _ => 0,
            },
            1245217292 => match prop {
                _ => 0,
            },
            1251058090 => match prop {
                _ => 0,
            },
            1252848954 => match prop {
                _ => 0,
            },
            1260505505 => match prop {
                _ => 0,
            },
            1260650574 => match prop {
                _ => 0,
            },
            1268542332 => match prop {
                _ => 0,
            },
            1281925730 => match prop {
                _ => 0,
            },
            1285652485 => match prop {
                _ => 0,
            },
            1287392070 => match prop {
                _ => 0,
            },
            1299126871 => match prop {
                _ => 0,
            },
            1300840506 => match prop {
                _ => 0,
            },
            1303795690 => match prop {
                _ => 0,
            },
            1304840413 => match prop {
                _ => 0,
            },
            1305183839 => match prop {
                _ => 0,
            },
            1307041759 => match prop {
                _ => 0,
            },
            1310608509 => match prop {
                _ => 0,
            },
            1329646415 => match prop {
                _ => 0,
            },
            1334484129 => match prop {
                _ => 0,
            },
            1335981549 => match prop {
                _ => 0,
            },
            1339347760 => match prop {
                _ => 0,
            },
            1351298697 => match prop {
                _ => 0,
            },
            1360408905 => match prop {
                _ => 0,
            },
            1377556343 => match prop {
                _ => 0,
            },
            1383045692 => match prop {
                _ => 0,
            },
            1387855156 => match prop {
                _ => 0,
            },
            1401173127 => match prop {
                _ => 0,
            },
            1402838566 => match prop {
                _ => 0,
            },
            1404847402 => match prop {
                _ => 0,
            },
            1411181986 => match prop {
                _ => 0,
            },
            1411407467 => match prop {
                _ => 0,
            },
            1412071761 => match prop {
                _ => 0,
            },
            1416205885 => match prop {
                _ => 0,
            },
            1417489154 => match prop {
                _ => 0,
            },
            1419761937 => match prop {
                _ => 0,
            },
            1423911732 => match prop {
                _ => 0,
            },
            1425443689 => match prop {
                _ => 0,
            },
            1426591983 => match prop {
                _ => 0,
            },
            1437502449 => match prop {
                _ => 0,
            },
            1437805879 => match prop {
                _ => 0,
            },
            1437953363 => match prop {
                _ => 0,
            },
            1447204868 => match prop {
                _ => 0,
            },
            1451395588 => match prop {
                _ => 0,
            },
            1457835157 => match prop {
                _ => 0,
            },
            1462361463 => match prop {
                _ => 0,
            },
            1466758467 => match prop {
                _ => 0,
            },
            1469900589 => match prop {
                _ => 0,
            },
            1472233963 => match prop {
                _ => 0,
            },
            1482703590 => match prop {
                _ => 0,
            },
            1482959167 => match prop {
                _ => 0,
            },
            1484403080 => match prop {
                _ => 0,
            },
            1485152156 => match prop {
                _ => 0,
            },
            1507914824 => match prop {
                _ => 0,
            },
            1509187699 => match prop {
                _ => 0,
            },
            1509553395 => match prop {
                _ => 0,
            },
            1520743889 => match prop {
                _ => 0,
            },
            1521410863 => match prop {
                _ => 0,
            },
            1525564444 => match prop {
                _ => 0,
            },
            1529196076 => match prop {
                _ => 0,
            },
            1532957894 => match prop {
                _ => 0,
            },
            1534661035 => match prop {
                _ => 0,
            },
            1549132990 => match prop {
                _ => 0,
            },
            1560379544 => match prop {
                _ => 0,
            },
            1566485204 => match prop {
                _ => 0,
            },
            1580146022 => match prop {
                _ => 0,
            },
            1580310250 => match prop {
                _ => 0,
            },
            1585845231 => match prop {
                _ => 0,
            },
            1595516126 => match prop {
                _ => 0,
            },
            1597423693 => match prop {
                _ => 0,
            },
            1599208980 => match prop {
                _ => 0,
            },
            1600972822 => match prop {
                _ => 0,
            },
            1607154358 => match prop {
                _ => 0,
            },
            1608871552 => match prop {
                _ => 0,
            },
            1620046519 => match prop {
                _ => 0,
            },
            1621171031 => match prop {
                _ => 0,
            },
            1623761950 => match prop {
                _ => 0,
            },
            1628702193 => match prop {
                _ => 0,
            },
            1634111441 => match prop {
                _ => 0,
            },
            1635779807 => match prop {
                _ => 0,
            },
            1638771189 => match prop {
                _ => 0,
            },
            1640371178 => match prop {
                _ => 0,
            },
            1658829314 => match prop {
                _ => 0,
            },
            1660063152 => match prop {
                _ => 0,
            },
            1663979128 => match prop {
                _ => 0,
            },
            1674181508 => match prop {
                _ => 0,
            },
            1675464909 => match prop {
                _ => 0,
            },
            1677625105 => match prop {
                _ => 0,
            },
            1680319473 => match prop {
                _ => 0,
            },
            1682466193 => match prop {
                _ => 0,
            },
            1683148259 => match prop {
                _ => 0,
            },
            1687234759 => match prop {
                _ => 0,
            },
            1692211062 => match prop {
                _ => 0,
            },
            1704287377 => match prop {
                _ => 0,
            },
            1714330368 => match prop {
                _ => 0,
            },
            1735638870 => match prop {
                _ => 0,
            },
            1742049831 => match prop {
                _ => 0,
            },
            1758889154 => match prop {
                _ => 0,
            },
            1765591967 => match prop {
                _ => 0,
            },
            1768891740 => match prop {
                _ => 0,
            },
            1775413392 => match prop {
                _ => 0,
            },
            1783015770 => match prop {
                _ => 0,
            },
            1785450214 => match prop {
                _ => 0,
            },
            1806887404 => match prop {
                _ => 0,
            },
            1807405624 => match prop {
                _ => 0,
            },
            1809719519 => match prop {
                _ => 0,
            },
            1810631287 => match prop {
                _ => 0,
            },
            1815067380 => match prop {
                _ => 0,
            },
            1834744321 => match prop {
                _ => 0,
            },
            1838606355 => match prop {
                _ => 0,
            },
            1842657554 => match prop {
                _ => 0,
            },
            1847130766 => match prop {
                _ => 0,
            },
            1847252529 => match prop {
                _ => 0,
            },
            1856042241 => match prop {
                _ => 0,
            },
            1865459582 => match prop {
                _ => 0,
            },
            1871374353 => match prop {
                _ => 0,
            },
            1878645084 => match prop {
                _ => 0,
            },
            1883228015 => match prop {
                _ => 0,
            },
            1893162501 => match prop {
                _ => 0,
            },
            1898987631 => match prop {
                _ => 0,
            },
            1904799276 => match prop {
                _ => 0,
            },
            1907098498 => match prop {
                _ => 0,
            },
            1909888760 => match prop {
                _ => 0,
            },
            1911125066 => match prop {
                _ => 0,
            },
            1911478936 => match prop {
                _ => 0,
            },
            1916426348 => match prop {
                _ => 0,
            },
            1918398963 => match prop {
                _ => 0,
            },
            1935646853 => match prop {
                _ => 0,
            },
            1945004755 => match prop {
                _ => 0,
            },
            1950629157 => match prop {
                _ => 0,
            },
            1959218052 => match prop {
                _ => 0,
            },
            1967976161 => match prop {
                _ => 0,
            },
            1973038258 => match prop {
                _ => 0,
            },
            1973544240 => match prop {
                _ => 0,
            },
            1975003073 => match prop {
                _ => 0,
            },
            1981873012 => match prop {
                _ => 0,
            },
            1983826977 => match prop {
                _ => 0,
            },
            1999602285 => match prop {
                _ => 0,
            },
            2004835150 => match prop {
                _ => 0,
            },
            2016517767 => match prop {
                _ => 0,
            },
            2022407955 => match prop {
                _ => 0,
            },
            2022622350 => match prop {
                _ => 0,
            },
            2028607225 => match prop {
                _ => 0,
            },
            2030761528 => match prop {
                _ => 0,
            },
            2042790032 => match prop {
                _ => 0,
            },
            2043862942 => match prop {
                _ => 0,
            },
            2044713172 => match prop {
                _ => 0,
            },
            2047409740 => match prop {
                _ => 0,
            },
            2056796094 => match prop {
                _ => 0,
            },
            2058353004 => match prop {
                _ => 0,
            },
            2059837836 => match prop {
                _ => 0,
            },
            2063403501 => match prop {
                _ => 0,
            },
            2067069095 => match prop {
                _ => 0,
            },
            2068733104 => match prop {
                _ => 0,
            },
            2069777674 => match prop {
                _ => 0,
            },
            2077209135 => match prop {
                _ => 0,
            },
            2082059205 => match prop {
                _ => 0,
            },
            2090586900 => match prop {
                _ => 0,
            },
            2093928680 => match prop {
                _ => 0,
            },
            2095639259 => match prop {
                _ => 0,
            },
            2097647324 => match prop {
                _ => 0,
            },
            2107101300 => match prop {
                _ => 0,
            },
            2108223431 => match prop {
                _ => 0,
            },
            2127690289 => match prop {
                _ => 0,
            },
            2133299955 => match prop {
                _ => 0,
            },
            2143335405 => match prop {
                _ => 0,
            },
            2147822146 => match prop {
                _ => 0,
            },
            2157484638 => match prop {
                _ => 0,
            },
            2162789131 => match prop {
                _ => 0,
            },
            2176052936 => match prop {
                _ => 0,
            },
            2185764099 => match prop {
                _ => 0,
            },
            2188021234 => match prop {
                _ => 0,
            },
            2188180465 => match prop {
                _ => 0,
            },
            2197970202 => match prop {
                _ => 0,
            },
            2199411900 => match prop {
                _ => 0,
            },
            2205249479 => match prop {
                _ => 0,
            },
            2218152070 => match prop {
                _ => 0,
            },
            2223149337 => match prop {
                _ => 0,
            },
            2226359599 => match prop {
                _ => 0,
            },
            2233826070 => match prop {
                _ => 0,
            },
            2235152071 => match prop {
                _ => 0,
            },
            2242383968 => match prop {
                _ => 0,
            },
            2247615214 => match prop {
                _ => 0,
            },
            2250791053 => match prop {
                _ => 0,
            },
            2251480897 => match prop {
                _ => 0,
            },
            2254336722 => match prop {
                _ => 0,
            },
            2262370178 => match prop {
                _ => 0,
            },
            2272882330 => match prop {
                _ => 0,
            },
            2273995522 => match prop {
                _ => 0,
            },
            2294589976 => match prop {
                _ => 0,
            },
            2295281155 => match prop {
                _ => 0,
            },
            2296667514 => match prop {
                _ => 0,
            },
            2297155007 => match prop {
                _ => 0,
            },
            2301859152 => match prop {
                _ => 0,
            },
            2310774935 => match prop {
                _ => 0,
            },
            2315554128 => match prop {
                _ => 0,
            },
            2320036040 => match prop {
                _ => 0,
            },
            2323601079 => match prop {
                _ => 0,
            },
            2324767716 => match prop {
                _ => 0,
            },
            2341007311 => match prop {
                _ => 0,
            },
            2347385850 => match prop {
                _ => 0,
            },
            2347447852 => match prop {
                _ => 0,
            },
            2347495698 => match prop {
                _ => 0,
            },
            2367409068 => match prop {
                _ => 0,
            },
            2382730787 => match prop {
                _ => 0,
            },
            2387106220 => match prop {
                _ => 0,
            },
            2391368822 => match prop {
                _ => 0,
            },
            2391383451 => match prop {
                _ => 0,
            },
            2391406946 => match prop {
                _ => 0,
            },
            2397081782 => match prop {
                _ => 0,
            },
            2405470396 => match prop {
                _ => 0,
            },
            2415094496 => match prop {
                _ => 0,
            },
            2417008758 => match prop {
                _ => 0,
            },
            2417041796 => match prop {
                _ => 0,
            },
            2433181523 => match prop {
                _ => 0,
            },
            2439245199 => match prop {
                _ => 0,
            },
            2445595289 => match prop {
                _ => 0,
            },
            2453401579 => match prop {
                _ => 0,
            },
            2461110595 => match prop {
                _ => 0,
            },
            2473145415 => match prop {
                _ => 0,
            },
            2474470126 => match prop {
                _ => 0,
            },
            2481509218 => match prop {
                _ => 0,
            },
            2483315170 => match prop {
                _ => 0,
            },
            2485617015 => match prop {
                _ => 0,
            },
            2489546625 => match prop {
                _ => 0,
            },
            2495723537 => match prop {
                _ => 0,
            },
            2506170314 => match prop {
                _ => 0,
            },
            2510884976 => match prop {
                _ => 0,
            },
            2513912981 => match prop {
                _ => 0,
            },
            2515109513 => match prop {
                _ => 0,
            },
            2519244187 => match prop {
                _ => 0,
            },
            2525727697 => match prop {
                _ => 0,
            },
            2529465313 => match prop {
                _ => 0,
            },
            2533589738 => match prop {
                _ => 0,
            },
            2542286263 => match prop {
                _ => 0,
            },
            2543172580 => match prop {
                _ => 0,
            },
            2551354335 => match prop {
                _ => 0,
            },
            2552916305 => match prop {
                _ => 0,
            },
            2556980723 => match prop {
                _ => 0,
            },
            2559016684 => match prop {
                _ => 0,
            },
            2559216714 => match prop {
                _ => 0,
            },
            2565941209 => match prop {
                _ => 0,
            },
            2571569899 => match prop {
                _ => 0,
            },
            2572171363 => match prop {
                _ => 0,
            },
            2574617495 => match prop {
                _ => 0,
            },
            2581212453 => match prop {
                _ => 0,
            },
            2590856083 => match prop {
                _ => 0,
            },
            2597039031 => match prop {
                _ => 0,
            },
            2598011224 => match prop {
                _ => 0,
            },
            2601014836 => match prop {
                _ => 0,
            },
            2603310189 => match prop {
                _ => 0,
            },
            2604431987 => match prop {
                _ => 0,
            },
            2609359061 => match prop {
                _ => 0,
            },
            2611217952 => match prop {
                _ => 0,
            },
            2614616156 => match prop {
                _ => 0,
            },
            2624227202 => match prop {
                _ => 0,
            },
            2629017746 => match prop {
                _ => 0,
            },
            2635815018 => match prop {
                _ => 0,
            },
            2636378356 => match prop {
                _ => 0,
            },
            2652556860 => match prop {
                _ => 0,
            },
            2655187982 => match prop {
                _ => 0,
            },
            2655215786 => match prop {
                _ => 0,
            },
            2665983363 => match prop {
                _ => 0,
            },
            2668620305 => match prop {
                _ => 0,
            },
            2674252688 => match prop {
                _ => 0,
            },
            2705031697 => match prop {
                _ => 0,
            },
            2706460486 => match prop {
                _ => 0,
            },
            2706606064 => match prop {
                _ => 0,
            },
            2706619895 => match prop {
                _ => 0,
            },
            2713105998 => match prop {
                _ => 0,
            },
            2713554722 => match prop {
                _ => 0,
            },
            2715220739 => match prop {
                _ => 0,
            },
            2728634034 => match prop {
                _ => 0,
            },
            2732653382 => match prop {
                _ => 0,
            },
            2736907675 => match prop {
                _ => 0,
            },
            2740243338 => match prop {
                _ => 0,
            },
            2744685151 => match prop {
                _ => 0,
            },
            2752243245 => match prop {
                _ => 0,
            },
            2757150158 => match prop {
                _ => 0,
            },
            2759199220 => match prop {
                _ => 0,
            },
            2769231204 => match prop {
                _ => 0,
            },
            2770003689 => match prop {
                _ => 0,
            },
            2771591690 => match prop {
                _ => 0,
            },
            2775532180 => match prop {
                _ => 0,
            },
            2777663545 => match prop {
                _ => 0,
            },
            2778083089 => match prop {
                _ => 0,
            },
            2781568857 => match prop {
                _ => 0,
            },
            2798486643 => match prop {
                _ => 0,
            },
            2799835756 => match prop {
                _ => 0,
            },
            2802773753 => match prop {
                _ => 0,
            },
            2802850158 => match prop {
                _ => 0,
            },
            2804161546 => match prop {
                _ => 0,
            },
            2809605785 => match prop {
                _ => 0,
            },
            2814081492 => match prop {
                _ => 0,
            },
            2816379211 => match prop {
                _ => 0,
            },
            2827207264 => match prop {
                _ => 0,
            },
            2827736869 => match prop {
                _ => 0,
            },
            2830218821 => match prop {
                _ => 0,
            },
            2835456948 => match prop {
                _ => 0,
            },
            2837617999 => match prop {
                _ => 0,
            },
            2839578677 => match prop {
                _ => 0,
            },
            2852063980 => match prop {
                _ => 0,
            },
            2853485674 => match prop {
                _ => 0,
            },
            2857406711 => match prop {
                _ => 0,
            },
            2859738748 => match prop {
                _ => 0,
            },
            2874132201 => match prop {
                _ => 0,
            },
            2887950389 => match prop {
                _ => 0,
            },
            2889183280 => match prop {
                _ => 0,
            },
            2893384427 => match prop {
                _ => 0,
            },
            2898889636 => match prop {
                _ => 0,
            },
            2904328755 => match prop {
                _ => 0,
            },
            2906023776 => match prop {
                _ => 0,
            },
            2914609552 => match prop {
                _ => 0,
            },
            2916149573 => match prop {
                _ => 0,
            },
            2924175390 => match prop {
                _ => 0,
            },
            2934153892 => match prop {
                _ => 0,
            },
            2937912522 => match prop {
                _ => 0,
            },
            2938176219 => match prop {
                _ => 0,
            },
            2943643501 => match prop {
                _ => 0,
            },
            2945172077 => match prop {
                _ => 0,
            },
            2949456006 => match prop {
                _ => 0,
            },
            2951183804 => match prop {
                _ => 0,
            },
            2954562838 => match prop {
                _ => 0,
            },
            2963535650 => match prop {
                _ => 0,
            },
            2979338954 => match prop {
                _ => 0,
            },
            2986769608 => match prop {
                _ => 0,
            },
            2998442950 => match prop {
                _ => 0,
            },
            3001207471 => match prop {
                _ => 0,
            },
            3008276851 => match prop {
                _ => 0,
            },
            3008791417 => match prop {
                _ => 0,
            },
            3009204131 => match prop {
                _ => 0,
            },
            3009222698 => match prop {
                _ => 0,
            },
            3020489413 => match prop {
                _ => 0,
            },
            3021840470 => match prop {
                _ => 0,
            },
            3024970846 => match prop {
                _ => 0,
            },
            3026737570 => match prop {
                _ => 0,
            },
            3027567501 => match prop {
                _ => 0,
            },
            3027962421 => match prop {
                _ => 0,
            },
            3040386961 => match prop {
                _ => 0,
            },
            3041715199 => match prop {
                _ => 0,
            },
            3049322572 => match prop {
                _ => 0,
            },
            3050246964 => match prop {
                _ => 0,
            },
            3053780830 => match prop {
                _ => 0,
            },
            3057273783 => match prop {
                _ => 0,
            },
            3071757647 => match prop {
                _ => 0,
            },
            3079605661 => match prop {
                _ => 0,
            },
            3079942009 => match prop {
                _ => 0,
            },
            3081323446 => match prop {
                _ => 0,
            },
            3087945054 => match prop {
                _ => 0,
            },
            3101149627 => match prop {
                _ => 0,
            },
            3101698114 => match prop {
                _ => 0,
            },
            3112655638 => match prop {
                _ => 0,
            },
            3113134337 => match prop {
                _ => 0,
            },
            3119450353 => match prop {
                _ => 0,
            },
            3124254112 => match prop {
                _ => 0,
            },
            3124975700 => match prop {
                _ => 0,
            },
            3125803723 => match prop {
                _ => 0,
            },
            3127900445 => match prop {
                _ => 0,
            },
            3132237377 => match prop {
                _ => 0,
            },
            3136571912 => match prop {
                _ => 0,
            },
            3150382593 => match prop {
                _ => 0,
            },
            3171933400 => match prop {
                _ => 0,
            },
            3174744832 => match prop {
                _ => 0,
            },
            3179687236 => match prop {
                _ => 0,
            },
            3181161470 => match prop {
                _ => 0,
            },
            3190031847 => match prop {
                _ => 0,
            },
            3198132628 => match prop {
                _ => 0,
            },
            3200245327 => match prop {
                _ => 0,
            },
            3205830791 => match prop {
                _ => 0,
            },
            3206491090 => match prop {
                _ => 0,
            },
            3207858831 => match prop {
                _ => 0,
            },
            3219374653 => match prop {
                _ => 0,
            },
            3221913625 => match prop {
                _ => 0,
            },
            3242481149 => match prop {
                _ => 0,
            },
            3242617779 => match prop {
                _ => 0,
            },
            3243963512 => match prop {
                _ => 0,
            },
            3252649465 => match prop {
                _ => 0,
            },
            3256556792 => match prop {
                _ => 0,
            },
            3264961684 => match prop {
                _ => 0,
            },
            3265635763 => match prop {
                _ => 0,
            },
            3268803585 => match prop {
                _ => 0,
            },
            3277789161 => match prop {
                _ => 0,
            },
            3283111854 => match prop {
                _ => 0,
            },
            3285139300 => match prop {
                _ => 0,
            },
            3293443760 => match prop {
                _ => 0,
            },
            3293546465 => match prop {
                _ => 0,
            },
            3295246426 => match prop {
                _ => 0,
            },
            3296154744 => match prop {
                _ => 0,
            },
            3299480353 => match prop {
                _ => 0,
            },
            3303107099 => match prop {
                _ => 0,
            },
            3303938423 => match prop {
                _ => 0,
            },
            3304561284 => match prop {
                _ => 0,
            },
            3310460725 => match prop {
                _ => 0,
            },
            3313531582 => match prop {
                _ => 0,
            },
            3319311131 => match prop {
                _ => 0,
            },
            3327091369 => match prop {
                _ => 0,
            },
            3331915920 => match prop {
                _ => 0,
            },
            3342526732 => match prop {
                _ => 0,
            },
            3352864051 => match prop {
                _ => 0,
            },
            3355820592 => match prop {
                _ => 0,
            },
            3357820518 => match prop {
                _ => 0,
            },
            3367102660 => match prop {
                _ => 0,
            },
            3368373690 => match prop {
                _ => 0,
            },
            3377609919 => match prop {
                _ => 0,
            },
            3388369263 => match prop {
                _ => 0,
            },
            3390157468 => match prop {
                _ => 0,
            },
            3404854881 => match prop {
                _ => 0,
            },
            3406155212 => match prop {
                _ => 0,
            },
            3408363356 => match prop {
                _ => 0,
            },
            3413951693 => match prop {
                _ => 0,
            },
            3415622556 => match prop {
                _ => 0,
            },
            3419103109 => match prop {
                _ => 0,
            },
            3420628829 => match prop {
                _ => 0,
            },
            3422422726 => match prop {
                _ => 0,
            },
            3448662350 => match prop {
                _ => 0,
            },
            3451746338 => match prop {
                _ => 0,
            },
            3452421091 => match prop {
                _ => 0,
            },
            3454111270 => match prop {
                _ => 0,
            },
            3460190687 => match prop {
                _ => 0,
            },
            3473067441 => match prop {
                _ => 0,
            },
            3478079324 => match prop {
                _ => 0,
            },
            3486308946 => match prop {
                _ => 0,
            },
            3493046030 => match prop {
                _ => 0,
            },
            3495092785 => match prop {
                _ => 0,
            },
            3505215534 => match prop {
                _ => 0,
            },
            3508470533 => match prop {
                _ => 0,
            },
            3510044353 => match prop {
                _ => 0,
            },
            3512223829 => match prop {
                _ => 0,
            },
            3518393246 => match prop {
                _ => 0,
            },
            3521284610 => match prop {
                _ => 0,
            },
            3523091289 => match prop {
                _ => 0,
            },
            3544373492 => match prop {
                _ => 0,
            },
            3548104201 => match prop {
                _ => 0,
            },
            3566463478 => match prop {
                _ => 0,
            },
            3570813810 => match prop {
                _ => 0,
            },
            3571504051 => match prop {
                _ => 0,
            },
            3588315303 => match prop {
                _ => 0,
            },
            3590301190 => match prop {
                _ => 0,
            },
            3593883385 => match prop {
                _ => 0,
            },
            3611470254 => match prop {
                _ => 0,
            },
            3612865200 => match prop {
                _ => 0,
            },
            3615266464 => match prop {
                _ => 0,
            },
            3626867408 => match prop {
                _ => 0,
            },
            3630933823 => match prop {
                _ => 0,
            },
            3632507154 => match prop {
                _ => 0,
            },
            3640358203 => match prop {
                _ => 0,
            },
            3649129432 => match prop {
                _ => 0,
            },
            3650150729 => match prop {
                _ => 0,
            },
            3651124850 => match prop {
                _ => 0,
            },
            3657597509 => match prop {
                _ => 0,
            },
            3663146110 => match prop {
                _ => 0,
            },
            3678494232 => match prop {
                _ => 0,
            },
            3689010777 => match prop {
                _ => 0,
            },
            3692461612 => match prop {
                _ => 0,
            },
            3694346114 => match prop {
                _ => 0,
            },
            3698973494 => match prop {
                _ => 0,
            },
            3701648758 => match prop {
                _ => 0,
            },
            3708119000 => match prop {
                _ => 0,
            },
            3710013099 => match prop {
                _ => 0,
            },
            3724593414 => match prop {
                _ => 0,
            },
            3727388367 => match prop {
                _ => 0,
            },
            3732053477 => match prop {
                _ => 0,
            },
            3732776249 => match prop {
                _ => 0,
            },
            3736923433 => match prop {
                _ => 0,
            },
            3737207727 => match prop {
                _ => 0,
            },
            3740093272 => match prop {
                _ => 0,
            },
            3741457305 => match prop {
                _ => 0,
            },
            3747195512 => match prop {
                _ => 0,
            },
            3749851601 => match prop {
                _ => 0,
            },
            3758799889 => match prop {
                _ => 0,
            },
            3760055223 => match prop {
                _ => 0,
            },
            3765753017 => match prop {
                _ => 0,
            },
            3778827333 => match prop {
                _ => 0,
            },
            3798115385 => match prop {
                _ => 0,
            },
            3800577675 => match prop {
                _ => 0,
            },
            3812236995 => match prop {
                _ => 0,
            },
            3815607619 => match prop {
                _ => 0,
            },
            3821786052 => match prop {
                _ => 0,
            },
            3824725483 => match prop {
                _ => 0,
            },
            3825984169 => match prop {
                _ => 0,
            },
            3827777499 => match prop {
                _ => 0,
            },
            3840914261 => match prop {
                _ => 0,
            },
            3843373140 => match prop {
                _ => 0,
            },
            3849074793 => match prop {
                _ => 0,
            },
            3850581409 => match prop {
                _ => 0,
            },
            3856911033 => match prop {
                _ => 0,
            },
            3869604511 => match prop {
                _ => 0,
            },
            3875453745 => match prop {
                _ => 0,
            },
            3888040117 => match prop {
                _ => 0,
            },
            3893378262 => match prop {
                _ => 0,
            },
            3893394355 => match prop {
                _ => 0,
            },
            3895139033 => match prop {
                _ => 0,
            },
            3898045240 => match prop {
                _ => 0,
            },
            3900360178 => match prop {
                _ => 0,
            },
            3902619387 => match prop {
                _ => 0,
            },
            3905492369 => match prop {
                _ => 0,
            },
            3907093117 => match prop {
                _ => 0,
            },
            3915482550 => match prop {
                _ => 0,
            },
            3939117080 => match prop {
                _ => 0,
            },
            3940055652 => match prop {
                _ => 0,
            },
            3945020480 => match prop {
                _ => 0,
            },
            3946677679 => match prop {
                _ => 0,
            },
            3958052878 => match prop {
                _ => 0,
            },
            3958567839 => match prop {
                _ => 0,
            },
            3961806047 => match prop {
                _ => 0,
            },
            3967405729 => match prop {
                _ => 0,
            },
            3979015343 => match prop {
                _ => 0,
            },
            3982875396 => match prop {
                _ => 0,
            },
            4006246654 => match prop {
                _ => 0,
            },
            4009809668 => match prop {
                _ => 0,
            },
            4015995234 => match prop {
                _ => 0,
            },
            4017108033 => match prop {
                _ => 0,
            },
            4022376103 => match prop {
                _ => 0,
            },
            4024345920 => match prop {
                _ => 0,
            },
            4031249490 => match prop {
                _ => 0,
            },
            4037036970 => match prop {
                _ => 0,
            },
            4037862832 => match prop {
                _ => 0,
            },
            4074379575 => match prop {
                _ => 0,
            },
            4074543187 => match prop {
                _ => 0,
            },
            4086658281 => match prop {
                _ => 0,
            },
            4088093105 => match prop {
                _ => 0,
            },
            4095422895 => match prop {
                _ => 0,
            },
            4095574036 => match prop {
                _ => 0,
            },
            4095615324 => match prop {
                _ => 0,
            },
            4097777520 => match prop {
                _ => 0,
            },
            4105962743 => match prop {
                _ => 0,
            },
            4122056220 => match prop {
                _ => 0,
            },
            4123344466 => match prop {
                _ => 0,
            },
            4124623270 => match prop {
                _ => 0,
            },
            4124788165 => match prop {
                _ => 0,
            },
            4136498852 => match prop {
                _ => 0,
            },
            4142052618 => match prop {
                _ => 0,
            },
            4143007308 => match prop {
                _ => 0,
            },
            4148101412 => match prop {
                _ => 0,
            },
            4156078855 => match prop {
                _ => 0,
            },
            4158566097 => match prop {
                _ => 0,
            },
            4162380809 => match prop {
                _ => 0,
            },
            4165799628 => match prop {
                _ => 0,
            },
            4166981789 => match prop {
                _ => 0,
            },
            4175244083 => match prop {
                _ => 0,
            },
            4182860854 => match prop {
                _ => 0,
            },
            4186316022 => match prop {
                _ => 0,
            },
            4201705270 => match prop {
                _ => 0,
            },
            4207607924 => match prop {
                _ => 0,
            },
            4208778838 => match prop {
                _ => 0,
            },
            4217484030 => match prop {
                _ => 0,
            },
            4218914973 => match prop {
                _ => 0,
            },
            4219587988 => match prop {
                _ => 0,
            },
            4222183408 => match prop {
                _ => 0,
            },
            4231323485 => match prop {
                _ => 0,
            },
            4237592921 => match prop {
                _ => 0,
            },
            4238390223 => match prop {
                _ => 0,
            },
            4240577450 => match prop {
                _ => 0,
            },
            4243806635 => match prop {
                _ => 0,
            },
            4251960020 => match prop {
                _ => 0,
            },
            4252922144 => match prop {
                _ => 0,
            },
            4261334040 => match prop {
                _ => 0,
            },
            4266656042 => match prop {
                _ => 0,
            },
            4278684876 => match prop {
                _ => 0,
            },
            4278956645 => match prop {
                _ => 0,
            },
            4282788508 => match prop {
                _ => 0,
            },
            4288193352 => match prop {
                _ => 0,
            },
            4288270099 => match prop {
                _ => 0,
            },
            4292641817 => match prop {
                _ => 0,
            },
            4294318154 => match prop {
                _ => 0,
            },
            _ => 0,
        },
        IFC_SCHEMA::IFC4X3 => match type_code {
            0 => match prop {
                _ => 0,
            },
            1 => match prop {
                _ => 0,
            },
            2 => match prop {
                _ => 0,
            },
            3 => match prop {
                _ => 0,
            },
            4 => match prop {
                _ => 0,
            },
            5 => match prop {
                _ => 0,
            },
            6 => match prop {
                _ => 0,
            },
            7 => match prop {
                _ => 0,
            },
            8 => match prop {
                _ => 0,
            },
            9 => match prop {
                _ => 0,
            },
            10 => match prop {
                _ => 0,
            },
            11 => match prop {
                _ => 0,
            },
            12 => match prop {
                _ => 0,
            },
            13 => match prop {
                _ => 0,
            },
            14 => match prop {
                _ => 0,
            },
            15 => match prop {
                _ => 0,
            },
            16 => match prop {
                _ => 0,
            },
            17 => match prop {
                _ => 0,
            },
            18 => match prop {
                _ => 0,
            },
            19 => match prop {
                _ => 0,
            },
            20 => match prop {
                _ => 0,
            },
            5716631 => match prop {
                _ => 0,
            },
            15328376 => match prop {
                _ => 0,
            },
            24185140 => match prop {
                _ => 0,
            },
            24726584 => match prop {
                _ => 0,
            },
            25142252 => match prop {
                _ => 0,
            },
            32344328 => match prop {
                _ => 0,
            },
            32440307 => match prop {
                _ => 0,
            },
            33720170 => match prop {
                _ => 0,
            },
            39481116 => match prop {
                _ => 0,
            },
            42703149 => match prop {
                _ => 0,
            },
            45288368 => match prop {
                _ => 0,
            },
            59481748 => match prop {
                _ => 0,
            },
            76236018 => match prop {
                _ => 0,
            },
            90941305 => match prop {
                _ => 0,
            },
            101040310 => match prop {
                _ => 0,
            },
            103090709 => match prop {
                _ => 0,
            },
            110355661 => match prop {
                _ => 0,
            },
            125510826 => match prop {
                _ => 0,
            },
            130549933 => match prop {
                _ => 0,
            },
            132023988 => match prop {
                _ => 0,
            },
            144952367 => match prop {
                _ => 0,
            },
            146592293 => match prop {
                _ => 0,
            },
            148013059 => match prop {
                _ => 0,
            },
            148025276 => match prop {
                _ => 0,
            },
            160246688 => match prop {
                _ => 0,
            },
            164193824 => match prop {
                _ => 0,
            },
            167062518 => match prop {
                _ => 0,
            },
            177149247 => match prop {
                _ => 0,
            },
            178086475 => match prop {
                _ => 0,
            },
            178912537 => match prop {
                _ => 0,
            },
            180925521 => match prop {
                _ => 0,
            },
            182550632 => match prop {
                _ => 0,
            },
            182646315 => match prop {
                _ => 0,
            },
            205026976 => match prop {
                _ => 0,
            },
            211053100 => match prop {
                _ => 0,
            },
            214636428 => match prop {
                _ => 0,
            },
            219451334 => match prop {
                _ => 0,
            },
            220341763 => match prop {
                _ => 0,
            },
            222769930 => match prop {
                _ => 0,
            },
            230924584 => match prop {
                _ => 0,
            },
            231477066 => match prop {
                _ => 0,
            },
            234836483 => match prop {
                _ => 0,
            },
            248100487 => match prop {
                _ => 0,
            },
            263784265 => match prop {
                _ => 0,
            },
            264262732 => match prop {
                _ => 0,
            },
            277319702 => match prop {
                _ => 0,
            },
            279856033 => match prop {
                _ => 0,
            },
            280115917 => match prop {
                _ => 0,
            },
            297599258 => match prop {
                _ => 0,
            },
            300633059 => match prop {
                _ => 0,
            },
            307848117 => match prop {
                _ => 0,
            },
            310824031 => match prop {
                _ => 0,
            },
            315944413 => match prop {
                _ => 0,
            },
            317615605 => match prop {
                _ => 0,
            },
            325726236 => match prop {
                _ => 0,
            },
            331165859 => match prop {
                _ => 0,
            },
            335055490 => match prop {
                _ => 0,
            },
            336235671 => match prop {
                _ => 0,
            },
            338393293 => match prop {
                _ => 0,
            },
            339256511 => match prop {
                _ => 0,
            },
            342316401 => match prop {
                _ => 0,
            },
            346874300 => match prop {
                _ => 0,
            },
            366585022 => match prop {
                _ => 0,
            },
            370225590 => match prop {
                _ => 0,
            },
            374418227 => match prop {
                _ => 0,
            },
            377706215 => match prop {
                _ => 0,
            },
            385403989 => match prop {
                _ => 0,
            },
            388784114 => match prop {
                _ => 0,
            },
            395041908 => match prop {
                _ => 0,
            },
            395920057 => match prop {
                _ => 0,
            },
            400855858 => match prop {
                _ => 0,
            },
            402227799 => match prop {
                _ => 0,
            },
            411424972 => match prop {
                _ => 0,
            },
            413509423 => match prop {
                _ => 0,
            },
            427810014 => match prop {
                _ => 0,
            },
            427948657 => match prop {
                _ => 0,
            },
            428585644 => match prop {
                _ => 0,
            },
            445594917 => match prop {
                _ => 0,
            },
            448429030 => match prop {
                _ => 0,
            },
            451544542 => match prop {
                _ => 0,
            },
            463610769 => match prop {
                _ => 0,
            },
            476780140 => match prop {
                _ => 0,
            },
            477187591 => match prop {
                _ => 0,
            },
            478536968 => match prop {
                _ => 0,
            },
            479945903 => match prop {
                _ => 0,
            },
            484807127 => match prop {
                _ => 0,
            },
            488727124 => match prop {
                _ => 0,
            },
            492091185 => match prop {
                _ => 0,
            },
            504942748 => match prop {
                _ => 0,
            },
            506776471 => match prop {
                _ => 0,
            },
            512836454 => match prop {
                _ => 0,
            },
            514975943 => match prop {
                _ => 0,
            },
            525669439 => match prop {
                _ => 0,
            },
            530289379 => match prop {
                _ => 0,
            },
            531007025 => match prop {
                _ => 0,
            },
            536804194 => match prop {
                _ => 0,
            },
            539742890 => match prop {
                _ => 0,
            },
            544395925 => match prop {
                _ => 0,
            },
            550521510 => match prop {
                _ => 0,
            },
            552965576 => match prop {
                _ => 0,
            },
            562808652 => match prop {
                _ => 0,
            },
            569719735 => match prop {
                _ => 0,
            },
            572779678 => match prop {
                _ => 0,
            },
            574549367 => match prop {
                _ => 0,
            },
            578613899 => match prop {
                _ => 0,
            },
            581633288 => match prop {
                _ => 0,
            },
            590820931 => match prop {
                _ => 0,
            },
            593015953 => match prop {
                _ => 0,
            },
            597895409 => match prop {
                _ => 0,
            },
            602808272 => match prop {
                _ => 0,
            },
            603570806 => match prop {
                _ => 0,
            },
            603775116 => match prop {
                _ => 0,
            },
            609421318 => match prop {
                _ => 0,
            },
            616511568 => match prop {
                _ => 0,
            },
            618182010 => match prop {
                _ => 0,
            },
            618700268 => match prop {
                _ => 0,
            },
            626085974 => match prop {
                _ => 0,
            },
            629592764 => match prop {
                _ => 0,
            },
            630975310 => match prop {
                _ => 0,
            },
            635142910 => match prop {
                _ => 0,
            },
            639361253 => match prop {
                _ => 0,
            },
            639542469 => match prop {
                _ => 0,
            },
            644574406 => match prop {
                _ => 0,
            },
            647756555 => match prop {
                _ => 0,
            },
            647927063 => match prop {
                _ => 0,
            },
            653396225 => match prop {
                _ => 0,
            },
            655969474 => match prop {
                _ => 0,
            },
            663422040 => match prop {
                _ => 0,
            },
            669184980 => match prop {
                _ => 0,
            },
            673634403 => match prop {
                _ => 0,
            },
            677532197 => match prop {
                _ => 0,
            },
            679976338 => match prop {
                _ => 0,
            },
            682877961 => match prop {
                _ => 0,
            },
            683857671 => match prop {
                _ => 0,
            },
            693640335 => match prop {
                _ => 0,
            },
            699246055 => match prop {
                _ => 0,
            },
            707683696 => match prop {
                _ => 0,
            },
            710110818 => match prop {
                _ => 0,
            },
            710998568 => match prop {
                _ => 0,
            },
            712377611 => match prop {
                _ => 0,
            },
            723233188 => match prop {
                _ => 0,
            },
            728799441 => match prop {
                _ => 0,
            },
            734778138 => match prop {
                _ => 0,
            },
            738039164 => match prop {
                _ => 0,
            },
            738692330 => match prop {
                _ => 0,
            },
            747523909 => match prop {
                _ => 0,
            },
            750771296 => match prop {
                _ => 0,
            },
            753842376 => match prop {
                _ => 0,
            },
            759155922 => match prop {
                _ => 0,
            },
            760658860 => match prop {
                _ => 0,
            },
            770865208 => match prop {
                _ => 0,
            },
            775493141 => match prop {
                _ => 0,
            },
            776857604 => match prop {
                _ => 0,
            },
            781010003 => match prop {
                _ => 0,
            },
            782932809 => match prop {
                _ => 0,
            },
            803316827 => match prop {
                _ => 0,
            },
            804291784 => match prop {
                _ => 0,
            },
            807026263 => match prop {
                _ => 0,
            },
            812098782 => match prop {
                _ => 0,
            },
            812556717 => match prop {
                _ => 0,
            },
            816062949 => match prop {
                _ => 0,
            },
            819412036 => match prop {
                _ => 0,
            },
            819618141 => match prop {
                _ => 0,
            },
            823603102 => match prop {
                _ => 0,
            },
            825690147 => match prop {
                _ => 0,
            },
            826625072 => match prop {
                _ => 0,
            },
            840318589 => match prop {
                _ => 0,
            },
            843113511 => match prop {
                _ => 0,
            },
            846575682 => match prop {
                _ => 0,
            },
            852622518 => match prop {
                _ => 0,
            },
            853536259 => match prop {
                _ => 0,
            },
            862014818 => match prop {
                _ => 0,
            },
            867548509 => match prop {
                _ => 0,
            },
            869906466 => match prop {
                _ => 0,
            },
            871118103 => match prop {
                _ => 0,
            },
            886880790 => match prop {
                _ => 0,
            },
            891718957 => match prop {
                _ => 0,
            },
            900683007 => match prop {
                _ => 0,
            },
            901063453 => match prop {
                _ => 0,
            },
            912023232 => match prop {
                _ => 0,
            },
            917726184 => match prop {
                _ => 0,
            },
            919958153 => match prop {
                _ => 0,
            },
            926996030 => match prop {
                _ => 0,
            },
            931644368 => match prop {
                _ => 0,
            },
            941946838 => match prop {
                _ => 0,
            },
            963979645 => match prop {
                _ => 0,
            },
            964333572 => match prop {
                _ => 0,
            },
            976884017 => match prop {
                _ => 0,
            },
            977012517 => match prop {
                _ => 0,
            },
            979691226 => match prop {
                _ => 0,
            },
            982818633 => match prop {
                _ => 0,
            },
            985171141 => match prop {
                _ => 0,
            },
            986844984 => match prop {
                _ => 0,
            },
            987401354 => match prop {
                _ => 0,
            },
            987898635 => match prop {
                _ => 0,
            },
            991950508 => match prop {
                _ => 0,
            },
            1003880860 => match prop {
                _ => 0,
            },
            1004757350 => match prop {
                _ => 0,
            },
            1008929658 => match prop {
                _ => 0,
            },
            1010789467 => match prop {
                _ => 0,
            },
            1027710054 => match prop {
                _ => 0,
            },
            1027922057 => match prop {
                _ => 0,
            },
            1028945134 => match prop {
                _ => 0,
            },
            1029017970 => match prop {
                _ => 0,
            },
            1033248425 => match prop {
                _ => 0,
            },
            1033361043 => match prop {
                _ => 0,
            },
            1039846685 => match prop {
                _ => 0,
            },
            1040185647 => match prop {
                _ => 0,
            },
            1042787934 => match prop {
                _ => 0,
            },
            1045800335 => match prop {
                _ => 0,
            },
            1051575348 => match prop {
                _ => 0,
            },
            1051757585 => match prop {
                _ => 0,
            },
            1052013943 => match prop {
                _ => 0,
            },
            1054537805 => match prop {
                _ => 0,
            },
            1060000209 => match prop {
                _ => 0,
            },
            1062813311 => match prop {
                _ => 0,
            },
            1072016465 => match prop {
                _ => 0,
            },
            1073191201 => match prop {
                _ => 0,
            },
            1076942058 => match prop {
                _ => 0,
            },
            1077100507 => match prop {
                _ => 0,
            },
            1095909175 => match prop {
                _ => 0,
            },
            1096409881 => match prop {
                _ => 0,
            },
            1105321065 => match prop {
                _ => 0,
            },
            1114901282 => match prop {
                _ => 0,
            },
            1123145078 => match prop {
                _ => 0,
            },
            1133259667 => match prop {
                _ => 0,
            },
            1136057603 => match prop {
                _ => 0,
            },
            1154170062 => match prop {
                _ => 0,
            },
            1154579445 => match prop {
                _ => 0,
            },
            1158309216 => match prop {
                _ => 0,
            },
            1161773419 => match prop {
                _ => 0,
            },
            1162798199 => match prop {
                _ => 0,
            },
            1175146630 => match prop {
                _ => 0,
            },
            1177604601 => match prop {
                _ => 0,
            },
            1179482911 => match prop {
                _ => 0,
            },
            1190533807 => match prop {
                _ => 0,
            },
            1199560280 => match prop {
                _ => 0,
            },
            1204542856 => match prop {
                _ => 0,
            },
            1207048766 => match prop {
                _ => 0,
            },
            1209101575 => match prop {
                _ => 0,
            },
            1210645708 => match prop {
                _ => 0,
            },
            1213902940 => match prop {
                _ => 0,
            },
            1217240411 => match prop {
                _ => 0,
            },
            1229763772 => match prop {
                _ => 0,
            },
            1232101972 => match prop {
                _ => 0,
            },
            1235345126 => match prop {
                _ => 0,
            },
            1236880293 => match prop {
                _ => 0,
            },
            1245217292 => match prop {
                _ => 0,
            },
            1251058090 => match prop {
                _ => 0,
            },
            1252848954 => match prop {
                _ => 0,
            },
            1260505505 => match prop {
                _ => 0,
            },
            1260650574 => match prop {
                _ => 0,
            },
            1268542332 => match prop {
                _ => 0,
            },
            1281925730 => match prop {
                _ => 0,
            },
            1285652485 => match prop {
                _ => 0,
            },
            1287392070 => match prop {
                _ => 0,
            },
            1290935644 => match prop {
                _ => 0,
            },
            1300840506 => match prop {
                _ => 0,
            },
            1303795690 => match prop {
                _ => 0,
            },
            1304840413 => match prop {
                _ => 0,
            },
            1305183839 => match prop {
                _ => 0,
            },
            1306400036 => match prop {
                _ => 0,
            },
            1307041759 => match prop {
                _ => 0,
            },
            1310608509 => match prop {
                _ => 0,
            },
            1310830890 => match prop {
                _ => 0,
            },
            1329646415 => match prop {
                _ => 0,
            },
            1334484129 => match prop {
                _ => 0,
            },
            1335981549 => match prop {
                _ => 0,
            },
            1339347760 => match prop {
                _ => 0,
            },
            1351298697 => match prop {
                _ => 0,
            },
            1356537516 => match prop {
                _ => 0,
            },
            1360408905 => match prop {
                _ => 0,
            },
            1377556343 => match prop {
                _ => 0,
            },
            1383045692 => match prop {
                _ => 0,
            },
            1383356374 => match prop {
                _ => 0,
            },
            1387855156 => match prop {
                _ => 0,
            },
            1401173127 => match prop {
                _ => 0,
            },
            1402838566 => match prop {
                _ => 0,
            },
            1404847402 => match prop {
                _ => 0,
            },
            1411181986 => match prop {
                _ => 0,
            },
            1411407467 => match prop {
                _ => 0,
            },
            1412071761 => match prop {
                _ => 0,
            },
            1416205885 => match prop {
                _ => 0,
            },
            1417489154 => match prop {
                _ => 0,
            },
            1419761937 => match prop {
                _ => 0,
            },
            1423911732 => match prop {
                _ => 0,
            },
            1425443689 => match prop {
                _ => 0,
            },
            1426591983 => match prop {
                _ => 0,
            },
            1437502449 => match prop {
                _ => 0,
            },
            1437805879 => match prop {
                _ => 0,
            },
            1437953363 => match prop {
                _ => 0,
            },
            1441486842 => match prop {
                _ => 0,
            },
            1447204868 => match prop {
                _ => 0,
            },
            1451395588 => match prop {
                _ => 0,
            },
            1457835157 => match prop {
                _ => 0,
            },
            1462361463 => match prop {
                _ => 0,
            },
            1466758467 => match prop {
                _ => 0,
            },
            1469900589 => match prop {
                _ => 0,
            },
            1472233963 => match prop {
                _ => 0,
            },
            1482703590 => match prop {
                _ => 0,
            },
            1482959167 => match prop {
                _ => 0,
            },
            1484403080 => match prop {
                _ => 0,
            },
            1485152156 => match prop {
                _ => 0,
            },
            1502416096 => match prop {
                _ => 0,
            },
            1507914824 => match prop {
                _ => 0,
            },
            1509187699 => match prop {
                _ => 0,
            },
            1509553395 => match prop {
                _ => 0,
            },
            1520743889 => match prop {
                _ => 0,
            },
            1521410863 => match prop {
                _ => 0,
            },
            1525564444 => match prop {
                _ => 0,
            },
            1529196076 => match prop {
                _ => 0,
            },
            1530820697 => match prop {
                _ => 0,
            },
            1532957894 => match prop {
                _ => 0,
            },
            1534661035 => match prop {
                _ => 0,
            },
            1545765605 => match prop {
                _ => 0,
            },
            1549132990 => match prop {
                _ => 0,
            },
            1560379544 => match prop {
                _ => 0,
            },
            1566485204 => match prop {
                _ => 0,
            },
            1580146022 => match prop {
                _ => 0,
            },
            1580310250 => match prop {
                _ => 0,
            },
            1585845231 => match prop {
                _ => 0,
            },
            1594536857 => match prop {
                _ => 0,
            },
            1595516126 => match prop {
                _ => 0,
            },
            1597423693 => match prop {
                _ => 0,
            },
            1599208980 => match prop {
                _ => 0,
            },
            1600972822 => match prop {
                _ => 0,
            },
            1607154358 => match prop {
                _ => 0,
            },
            1608871552 => match prop {
                _ => 0,
            },
            1620046519 => match prop {
                _ => 0,
            },
            1621171031 => match prop {
                _ => 0,
            },
            1623761950 => match prop {
                _ => 0,
            },
            1626504194 => match prop {
                _ => 0,
            },
            1628702193 => match prop {
                _ => 0,
            },
            1634111441 => match prop {
                _ => 0,
            },
            1635779807 => match prop {
                _ => 0,
            },
            1638771189 => match prop {
                _ => 0,
            },
            1638804497 => match prop {
                _ => 0,
            },
            1640371178 => match prop {
                _ => 0,
            },
            1658829314 => match prop {
                _ => 0,
            },
            1660063152 => match prop {
                _ => 0,
            },
            1662888072 => match prop {
                _ => 0,
            },
            1663979128 => match prop {
                _ => 0,
            },
            1674181508 => match prop {
                _ => 0,
            },
            1675464909 => match prop {
                _ => 0,
            },
            1677625105 => match prop {
                _ => 0,
            },
            1680319473 => match prop {
                _ => 0,
            },
            1682466193 => match prop {
                _ => 0,
            },
            1683148259 => match prop {
                _ => 0,
            },
            1687234759 => match prop {
                _ => 0,
            },
            1692211062 => match prop {
                _ => 0,
            },
            1704287377 => match prop {
                _ => 0,
            },
            1714330368 => match prop {
                _ => 0,
            },
            1735638870 => match prop {
                _ => 0,
            },
            1742049831 => match prop {
                _ => 0,
            },
            1758889154 => match prop {
                _ => 0,
            },
            1763565496 => match prop {
                _ => 0,
            },
            1765591967 => match prop {
                _ => 0,
            },
            1768891740 => match prop {
                _ => 0,
            },
            1770583370 => match prop {
                _ => 0,
            },
            1775413392 => match prop {
                _ => 0,
            },
            1783015770 => match prop {
                _ => 0,
            },
            1785450214 => match prop {
                _ => 0,
            },
            1794013214 => match prop {
                _ => 0,
            },
            1806887404 => match prop {
                _ => 0,
            },
            1807405624 => match prop {
                _ => 0,
            },
            1809719519 => match prop {
                _ => 0,
            },
            1810631287 => match prop {
                _ => 0,
            },
            1815067380 => match prop {
                _ => 0,
            },
            1834744321 => match prop {
                _ => 0,
            },
            1838606355 => match prop {
                _ => 0,
            },
            1842657554 => match prop {
                _ => 0,
            },
            1847130766 => match prop {
                _ => 0,
            },
            1847252529 => match prop {
                _ => 0,
            },
            1856042241 => match prop {
                _ => 0,
            },
            1862484736 => match prop {
                _ => 0,
            },
            1865459582 => match prop {
                _ => 0,
            },
            1871374353 => match prop {
                _ => 0,
            },
            1876633798 => match prop {
                _ => 0,
            },
            1878645084 => match prop {
                _ => 0,
            },
            1883228015 => match prop {
                _ => 0,
            },
            1891881377 => match prop {
                _ => 0,
            },
            1893162501 => match prop {
                _ => 0,
            },
            1894708472 => match prop {
                _ => 0,
            },
            1898987631 => match prop {
                _ => 0,
            },
            1904799276 => match prop {
                _ => 0,
            },
            1907098498 => match prop {
                _ => 0,
            },
            1909888760 => match prop {
                _ => 0,
            },
            1911125066 => match prop {
                _ => 0,
            },
            1916426348 => match prop {
                _ => 0,
            },
            1918398963 => match prop {
                _ => 0,
            },
            1935646853 => match prop {
                _ => 0,
            },
            1945004755 => match prop {
                _ => 0,
            },
            1946335990 => match prop {
                _ => 0,
            },
            1950438474 => match prop {
                _ => 0,
            },
            1953115116 => match prop {
                _ => 0,
            },
            1959218052 => match prop {
                _ => 0,
            },
            1967976161 => match prop {
                _ => 0,
            },
            1971632696 => match prop {
                _ => 0,
            },
            1973038258 => match prop {
                _ => 0,
            },
            1973544240 => match prop {
                _ => 0,
            },
            1975003073 => match prop {
                _ => 0,
            },
            1981873012 => match prop {
                _ => 0,
            },
            1983826977 => match prop {
                _ => 0,
            },
            1999602285 => match prop {
                _ => 0,
            },
            2000195564 => match prop {
                _ => 0,
            },
            2004835150 => match prop {
                _ => 0,
            },
            2016517767 => match prop {
                _ => 0,
            },
            2022407955 => match prop {
                _ => 0,
            },
            2022622350 => match prop {
                _ => 0,
            },
            2028607225 => match prop {
                _ => 0,
            },
            2030761528 => match prop {
                _ => 0,
            },
            2042790032 => match prop {
                _ => 0,
            },
            2043862942 => match prop {
                _ => 0,
            },
            2044713172 => match prop {
                _ => 0,
            },
            2047409740 => match prop {
                _ => 0,
            },
            2056796094 => match prop {
                _ => 0,
            },
            2058353004 => match prop {
                _ => 0,
            },
            2059837836 => match prop {
                _ => 0,
            },
            2063403501 => match prop {
                _ => 0,
            },
            2067069095 => match prop {
                _ => 0,
            },
            2068733104 => match prop {
                _ => 0,
            },
            2069777674 => match prop {
                _ => 0,
            },
            2077209135 => match prop {
                _ => 0,
            },
            2078563270 => match prop {
                _ => 0,
            },
            2082059205 => match prop {
                _ => 0,
            },
            2090586900 => match prop {
                _ => 0,
            },
            2093928680 => match prop {
                _ => 0,
            },
            2095639259 => match prop {
                _ => 0,
            },
            2097647324 => match prop {
                _ => 0,
            },
            2107101300 => match prop {
                _ => 0,
            },
            2108223431 => match prop {
                _ => 0,
            },
            2127690289 => match prop {
                _ => 0,
            },
            2133299955 => match prop {
                _ => 0,
            },
            2142170206 => match prop {
                _ => 0,
            },
            2143335405 => match prop {
                _ => 0,
            },
            2147822146 => match prop {
                _ => 0,
            },
            2157484638 => match prop {
                _ => 0,
            },
            2162789131 => match prop {
                _ => 0,
            },
            2165702409 => match prop {
                _ => 0,
            },
            2176052936 => match prop {
                _ => 0,
            },
            2176059722 => match prop {
                _ => 0,
            },
            2182337498 => match prop {
                _ => 0,
            },
            2185764099 => match prop {
                _ => 0,
            },
            2188021234 => match prop {
                _ => 0,
            },
            2188180465 => match prop {
                _ => 0,
            },
            2197970202 => match prop {
                _ => 0,
            },
            2199411900 => match prop {
                _ => 0,
            },
            2205249479 => match prop {
                _ => 0,
            },
            2218152070 => match prop {
                _ => 0,
            },
            2223149337 => match prop {
                _ => 0,
            },
            2226359599 => match prop {
                _ => 0,
            },
            2233826070 => match prop {
                _ => 0,
            },
            2235152071 => match prop {
                _ => 0,
            },
            2242383968 => match prop {
                _ => 0,
            },
            2247615214 => match prop {
                _ => 0,
            },
            2250791053 => match prop {
                _ => 0,
            },
            2251480897 => match prop {
                _ => 0,
            },
            2254336722 => match prop {
                _ => 0,
            },
            2262370178 => match prop {
                _ => 0,
            },
            2272882330 => match prop {
                _ => 0,
            },
            2273995522 => match prop {
                _ => 0,
            },
            2281632017 => match prop {
                _ => 0,
            },
            2294589976 => match prop {
                _ => 0,
            },
            2295281155 => match prop {
                _ => 0,
            },
            2296667514 => match prop {
                _ => 0,
            },
            2297155007 => match prop {
                _ => 0,
            },
            2301859152 => match prop {
                _ => 0,
            },
            2310774935 => match prop {
                _ => 0,
            },
            2315554128 => match prop {
                _ => 0,
            },
            2320036040 => match prop {
                _ => 0,
            },
            2323601079 => match prop {
                _ => 0,
            },
            2324767716 => match prop {
                _ => 0,
            },
            2341007311 => match prop {
                _ => 0,
            },
            2347385850 => match prop {
                _ => 0,
            },
            2347447852 => match prop {
                _ => 0,
            },
            2347495698 => match prop {
                _ => 0,
            },
            2367409068 => match prop {
                _ => 0,
            },
            2382730787 => match prop {
                _ => 0,
            },
            2387106220 => match prop {
                _ => 0,
            },
            2391368822 => match prop {
                _ => 0,
            },
            2391383451 => match prop {
                _ => 0,
            },
            2391406946 => match prop {
                _ => 0,
            },
            2397081782 => match prop {
                _ => 0,
            },
            2405470396 => match prop {
                _ => 0,
            },
            2415094496 => match prop {
                _ => 0,
            },
            2417008758 => match prop {
                _ => 0,
            },
            2433181523 => match prop {
                _ => 0,
            },
            2439245199 => match prop {
                _ => 0,
            },
            2445595289 => match prop {
                _ => 0,
            },
            2453401579 => match prop {
                _ => 0,
            },
            2461110595 => match prop {
                _ => 0,
            },
            2473145415 => match prop {
                _ => 0,
            },
            2474470126 => match prop {
                _ => 0,
            },
            2481509218 => match prop {
                _ => 0,
            },
            2483315170 => match prop {
                _ => 0,
            },
            2485617015 => match prop {
                _ => 0,
            },
            2485787929 => match prop {
                _ => 0,
            },
            2489546625 => match prop {
                _ => 0,
            },
            2495723537 => match prop {
                _ => 0,
            },
            2506170314 => match prop {
                _ => 0,
            },
            2510884976 => match prop {
                _ => 0,
            },
            2513912981 => match prop {
                _ => 0,
            },
            2515109513 => match prop {
                _ => 0,
            },
            2519244187 => match prop {
                _ => 0,
            },
            2525727697 => match prop {
                _ => 0,
            },
            2529465313 => match prop {
                _ => 0,
            },
            2533589738 => match prop {
                _ => 0,
            },
            2542286263 => match prop {
                _ => 0,
            },
            2543172580 => match prop {
                _ => 0,
            },
            2551354335 => match prop {
                _ => 0,
            },
            2552916305 => match prop {
                _ => 0,
            },
            2556980723 => match prop {
                _ => 0,
            },
            2559016684 => match prop {
                _ => 0,
            },
            2559216714 => match prop {
                _ => 0,
            },
            2565941209 => match prop {
                _ => 0,
            },
            2568555532 => match prop {
                _ => 0,
            },
            2571569899 => match prop {
                _ => 0,
            },
            2572171363 => match prop {
                _ => 0,
            },
            2574617495 => match prop {
                _ => 0,
            },
            2581212453 => match prop {
                _ => 0,
            },
            2590856083 => match prop {
                _ => 0,
            },
            2597039031 => match prop {
                _ => 0,
            },
            2598011224 => match prop {
                _ => 0,
            },
            2601014836 => match prop {
                _ => 0,
            },
            2603310189 => match prop {
                _ => 0,
            },
            2604431987 => match prop {
                _ => 0,
            },
            2609359061 => match prop {
                _ => 0,
            },
            2611217952 => match prop {
                _ => 0,
            },
            2614616156 => match prop {
                _ => 0,
            },
            2624227202 => match prop {
                _ => 0,
            },
            2629017746 => match prop {
                _ => 0,
            },
            2635815018 => match prop {
                _ => 0,
            },
            2636378356 => match prop {
                _ => 0,
            },
            2652556860 => match prop {
                _ => 0,
            },
            2655187982 => match prop {
                _ => 0,
            },
            2655215786 => match prop {
                _ => 0,
            },
            2665983363 => match prop {
                _ => 0,
            },
            2668620305 => match prop {
                _ => 0,
            },
            2674252688 => match prop {
                _ => 0,
            },
            2680139844 => match prop {
                _ => 0,
            },
            2691318326 => match prop {
                _ => 0,
            },
            2696325953 => match prop {
                _ => 0,
            },
            2705031697 => match prop {
                _ => 0,
            },
            2706460486 => match prop {
                _ => 0,
            },
            2706606064 => match prop {
                _ => 0,
            },
            2706619895 => match prop {
                _ => 0,
            },
            2713105998 => match prop {
                _ => 0,
            },
            2713554722 => match prop {
                _ => 0,
            },
            2713699986 => match prop {
                _ => 0,
            },
            2715220739 => match prop {
                _ => 0,
            },
            2728634034 => match prop {
                _ => 0,
            },
            2732653382 => match prop {
                _ => 0,
            },
            2735484536 => match prop {
                _ => 0,
            },
            2736907675 => match prop {
                _ => 0,
            },
            2740243338 => match prop {
                _ => 0,
            },
            2744685151 => match prop {
                _ => 0,
            },
            2752243245 => match prop {
                _ => 0,
            },
            2757150158 => match prop {
                _ => 0,
            },
            2759199220 => match prop {
                _ => 0,
            },
            2769231204 => match prop {
                _ => 0,
            },
            2770003689 => match prop {
                _ => 0,
            },
            2771591690 => match prop {
                _ => 0,
            },
            2775532180 => match prop {
                _ => 0,
            },
            2777663545 => match prop {
                _ => 0,
            },
            2778083089 => match prop {
                _ => 0,
            },
            2781568857 => match prop {
                _ => 0,
            },
            2798486643 => match prop {
                _ => 0,
            },
            2799835756 => match prop {
                _ => 0,
            },
            2802773753 => match prop {
                _ => 0,
            },
            2802850158 => match prop {
                _ => 0,
            },
            2804161546 => match prop {
                _ => 0,
            },
            2809605785 => match prop {
                _ => 0,
            },
            2814081492 => match prop {
                _ => 0,
            },
            2816379211 => match prop {
                _ => 0,
            },
            2827207264 => match prop {
                _ => 0,
            },
            2827736869 => match prop {
                _ => 0,
            },
            2830218821 => match prop {
                _ => 0,
            },
            2835456948 => match prop {
                _ => 0,
            },
            2837617999 => match prop {
                _ => 0,
            },
            2839578677 => match prop {
                _ => 0,
            },
            2852063980 => match prop {
                _ => 0,
            },
            2853485674 => match prop {
                _ => 0,
            },
            2857406711 => match prop {
                _ => 0,
            },
            2859738748 => match prop {
                _ => 0,
            },
            2874132201 => match prop {
                _ => 0,
            },
            2879124712 => match prop {
                _ => 0,
            },
            2887950389 => match prop {
                _ => 0,
            },
            2889183280 => match prop {
                _ => 0,
            },
            2893384427 => match prop {
                _ => 0,
            },
            2898700619 => match prop {
                _ => 0,
            },
            2898889636 => match prop {
                _ => 0,
            },
            2904328755 => match prop {
                _ => 0,
            },
            2914609552 => match prop {
                _ => 0,
            },
            2916149573 => match prop {
                _ => 0,
            },
            2924175390 => match prop {
                _ => 0,
            },
            2934153892 => match prop {
                _ => 0,
            },
            2937912522 => match prop {
                _ => 0,
            },
            2938176219 => match prop {
                _ => 0,
            },
            2940368186 => match prop {
                _ => 0,
            },
            2943643501 => match prop {
                _ => 0,
            },
            2945172077 => match prop {
                _ => 0,
            },
            2949456006 => match prop {
                _ => 0,
            },
            2951183804 => match prop {
                _ => 0,
            },
            2954562838 => match prop {
                _ => 0,
            },
            2963535650 => match prop {
                _ => 0,
            },
            2979338954 => match prop {
                _ => 0,
            },
            2986769608 => match prop {
                _ => 0,
            },
            2998442950 => match prop {
                _ => 0,
            },
            3001207471 => match prop {
                _ => 0,
            },
            3008276851 => match prop {
                _ => 0,
            },
            3008791417 => match prop {
                _ => 0,
            },
            3009204131 => match prop {
                _ => 0,
            },
            3009222698 => match prop {
                _ => 0,
            },
            3020489413 => match prop {
                _ => 0,
            },
            3021840470 => match prop {
                _ => 0,
            },
            3024970846 => match prop {
                _ => 0,
            },
            3026737570 => match prop {
                _ => 0,
            },
            3027567501 => match prop {
                _ => 0,
            },
            3040386961 => match prop {
                _ => 0,
            },
            3041715199 => match prop {
                _ => 0,
            },
            3049322572 => match prop {
                _ => 0,
            },
            3050246964 => match prop {
                _ => 0,
            },
            3053780830 => match prop {
                _ => 0,
            },
            3057273783 => match prop {
                _ => 0,
            },
            3071239417 => match prop {
                _ => 0,
            },
            3071757647 => match prop {
                _ => 0,
            },
            3079605661 => match prop {
                _ => 0,
            },
            3081323446 => match prop {
                _ => 0,
            },
            3087945054 => match prop {
                _ => 0,
            },
            3101149627 => match prop {
                _ => 0,
            },
            3101698114 => match prop {
                _ => 0,
            },
            3112655638 => match prop {
                _ => 0,
            },
            3113134337 => match prop {
                _ => 0,
            },
            3119450353 => match prop {
                _ => 0,
            },
            3124254112 => match prop {
                _ => 0,
            },
            3124975700 => match prop {
                _ => 0,
            },
            3125803723 => match prop {
                _ => 0,
            },
            3132237377 => match prop {
                _ => 0,
            },
            3136571912 => match prop {
                _ => 0,
            },
            3150382593 => match prop {
                _ => 0,
            },
            3171933400 => match prop {
                _ => 0,
            },
            3174744832 => match prop {
                _ => 0,
            },
            3179687236 => match prop {
                _ => 0,
            },
            3181161470 => match prop {
                _ => 0,
            },
            3190031847 => match prop {
                _ => 0,
            },
            3198132628 => match prop {
                _ => 0,
            },
            3200245327 => match prop {
                _ => 0,
            },
            3203706013 => match prop {
                _ => 0,
            },
            3205830791 => match prop {
                _ => 0,
            },
            3206491090 => match prop {
                _ => 0,
            },
            3207858831 => match prop {
                _ => 0,
            },
            3221913625 => match prop {
                _ => 0,
            },
            3242617779 => match prop {
                _ => 0,
            },
            3243963512 => match prop {
                _ => 0,
            },
            3252649465 => match prop {
                _ => 0,
            },
            3256556792 => match prop {
                _ => 0,
            },
            3264961684 => match prop {
                _ => 0,
            },
            3265635763 => match prop {
                _ => 0,
            },
            3268803585 => match prop {
                _ => 0,
            },
            3277789161 => match prop {
                _ => 0,
            },
            3283111854 => match prop {
                _ => 0,
            },
            3285139300 => match prop {
                _ => 0,
            },
            3290496277 => match prop {
                _ => 0,
            },
            3293443760 => match prop {
                _ => 0,
            },
            3293546465 => match prop {
                _ => 0,
            },
            3295246426 => match prop {
                _ => 0,
            },
            3296154744 => match prop {
                _ => 0,
            },
            3303107099 => match prop {
                _ => 0,
            },
            3303938423 => match prop {
                _ => 0,
            },
            3304561284 => match prop {
                _ => 0,
            },
            3310460725 => match prop {
                _ => 0,
            },
            3313531582 => match prop {
                _ => 0,
            },
            3314249567 => match prop {
                _ => 0,
            },
            3319311131 => match prop {
                _ => 0,
            },
            3327091369 => match prop {
                _ => 0,
            },
            3331915920 => match prop {
                _ => 0,
            },
            3342526732 => match prop {
                _ => 0,
            },
            3352864051 => match prop {
                _ => 0,
            },
            3355820592 => match prop {
                _ => 0,
            },
            3357820518 => match prop {
                _ => 0,
            },
            3367102660 => match prop {
                _ => 0,
            },
            3368373690 => match prop {
                _ => 0,
            },
            3376911765 => match prop {
                _ => 0,
            },
            3377609919 => match prop {
                _ => 0,
            },
            3381221214 => match prop {
                _ => 0,
            },
            3388369263 => match prop {
                _ => 0,
            },
            3390157468 => match prop {
                _ => 0,
            },
            3404854881 => match prop {
                _ => 0,
            },
            3406155212 => match prop {
                _ => 0,
            },
            3408363356 => match prop {
                _ => 0,
            },
            3413951693 => match prop {
                _ => 0,
            },
            3415622556 => match prop {
                _ => 0,
            },
            3419103109 => match prop {
                _ => 0,
            },
            3420628829 => match prop {
                _ => 0,
            },
            3422422726 => match prop {
                _ => 0,
            },
            3425423356 => match prop {
                _ => 0,
            },
            3425753595 => match prop {
                _ => 0,
            },
            3426335179 => match prop {
                _ => 0,
            },
            3448662350 => match prop {
                _ => 0,
            },
            3451746338 => match prop {
                _ => 0,
            },
            3452421091 => match prop {
                _ => 0,
            },
            3454111270 => match prop {
                _ => 0,
            },
            3460190687 => match prop {
                _ => 0,
            },
            3460952963 => match prop {
                _ => 0,
            },
            3465909080 => match prop {
                _ => 0,
            },
            3473067441 => match prop {
                _ => 0,
            },
            3478079324 => match prop {
                _ => 0,
            },
            3486308946 => match prop {
                _ => 0,
            },
            3493046030 => match prop {
                _ => 0,
            },
            3495092785 => match prop {
                _ => 0,
            },
            3497074424 => match prop {
                _ => 0,
            },
            3505215534 => match prop {
                _ => 0,
            },
            3508470533 => match prop {
                _ => 0,
            },
            3510044353 => match prop {
                _ => 0,
            },
            3512223829 => match prop {
                _ => 0,
            },
            3518393246 => match prop {
                _ => 0,
            },
            3521284610 => match prop {
                _ => 0,
            },
            3523091289 => match prop {
                _ => 0,
            },
            3544373492 => match prop {
                _ => 0,
            },
            3548104201 => match prop {
                _ => 0,
            },
            3566463478 => match prop {
                _ => 0,
            },
            3570813810 => match prop {
                _ => 0,
            },
            3571504051 => match prop {
                _ => 0,
            },
            3588315303 => match prop {
                _ => 0,
            },
            3590301190 => match prop {
                _ => 0,
            },
            3593883385 => match prop {
                _ => 0,
            },
            3599934289 => match prop {
                _ => 0,
            },
            3611470254 => match prop {
                _ => 0,
            },
            3612865200 => match prop {
                _ => 0,
            },
            3615266464 => match prop {
                _ => 0,
            },
            3626867408 => match prop {
                _ => 0,
            },
            3630933823 => match prop {
                _ => 0,
            },
            3632507154 => match prop {
                _ => 0,
            },
            3633395639 => match prop {
                _ => 0,
            },
            3640358203 => match prop {
                _ => 0,
            },
            3649129432 => match prop {
                _ => 0,
            },
            3649138523 => match prop {
                _ => 0,
            },
            3649235739 => match prop {
                _ => 0,
            },
            3650150729 => match prop {
                _ => 0,
            },
            3651124850 => match prop {
                _ => 0,
            },
            3651464721 => match prop {
                _ => 0,
            },
            3657597509 => match prop {
                _ => 0,
            },
            3663046924 => match prop {
                _ => 0,
            },
            3663146110 => match prop {
                _ => 0,
            },
            3665877780 => match prop {
                _ => 0,
            },
            3678494232 => match prop {
                _ => 0,
            },
            3689010777 => match prop {
                _ => 0,
            },
            3692461612 => match prop {
                _ => 0,
            },
            3693000487 => match prop {
                _ => 0,
            },
            3694346114 => match prop {
                _ => 0,
            },
            3698973494 => match prop {
                _ => 0,
            },
            3701648758 => match prop {
                _ => 0,
            },
            3708119000 => match prop {
                _ => 0,
            },
            3710013099 => match prop {
                _ => 0,
            },
            3724593414 => match prop {
                _ => 0,
            },
            3727388367 => match prop {
                _ => 0,
            },
            3732053477 => match prop {
                _ => 0,
            },
            3732776249 => match prop {
                _ => 0,
            },
            3736923433 => match prop {
                _ => 0,
            },
            3737207727 => match prop {
                _ => 0,
            },
            3740093272 => match prop {
                _ => 0,
            },
            3741457305 => match prop {
                _ => 0,
            },
            3747195512 => match prop {
                _ => 0,
            },
            3749851601 => match prop {
                _ => 0,
            },
            3752311538 => match prop {
                _ => 0,
            },
            3758799889 => match prop {
                _ => 0,
            },
            3760055223 => match prop {
                _ => 0,
            },
            3765753017 => match prop {
                _ => 0,
            },
            3778827333 => match prop {
                _ => 0,
            },
            3798115385 => match prop {
                _ => 0,
            },
            3798194928 => match prop {
                _ => 0,
            },
            3800577675 => match prop {
                _ => 0,
            },
            3812236995 => match prop {
                _ => 0,
            },
            3815607619 => match prop {
                _ => 0,
            },
            3818125796 => match prop {
                _ => 0,
            },
            3821786052 => match prop {
                _ => 0,
            },
            3824725483 => match prop {
                _ => 0,
            },
            3825984169 => match prop {
                _ => 0,
            },
            3827777499 => match prop {
                _ => 0,
            },
            3840914261 => match prop {
                _ => 0,
            },
            3843373140 => match prop {
                _ => 0,
            },
            3849074793 => match prop {
                _ => 0,
            },
            3850581409 => match prop {
                _ => 0,
            },
            3856911033 => match prop {
                _ => 0,
            },
            3862327254 => match prop {
                _ => 0,
            },
            3869604511 => match prop {
                _ => 0,
            },
            3875453745 => match prop {
                _ => 0,
            },
            3888040117 => match prop {
                _ => 0,
            },
            3893378262 => match prop {
                _ => 0,
            },
            3893394355 => match prop {
                _ => 0,
            },
            3895139033 => match prop {
                _ => 0,
            },
            3898045240 => match prop {
                _ => 0,
            },
            3900360178 => match prop {
                _ => 0,
            },
            3902619387 => match prop {
                _ => 0,
            },
            3905492369 => match prop {
                _ => 0,
            },
            3907093117 => match prop {
                _ => 0,
            },
            3915482550 => match prop {
                _ => 0,
            },
            3939117080 => match prop {
                _ => 0,
            },
            3940055652 => match prop {
                _ => 0,
            },
            3945020480 => match prop {
                _ => 0,
            },
            3946677679 => match prop {
                _ => 0,
            },
            3948183225 => match prop {
                _ => 0,
            },
            3956297820 => match prop {
                _ => 0,
            },
            3958052878 => match prop {
                _ => 0,
            },
            3958567839 => match prop {
                _ => 0,
            },
            3961806047 => match prop {
                _ => 0,
            },
            3967405729 => match prop {
                _ => 0,
            },
            3979015343 => match prop {
                _ => 0,
            },
            3982875396 => match prop {
                _ => 0,
            },
            3992365140 => match prop {
                _ => 0,
            },
            3999819293 => match prop {
                _ => 0,
            },
            4006246654 => match prop {
                _ => 0,
            },
            4009809668 => match prop {
                _ => 0,
            },
            4015995234 => match prop {
                _ => 0,
            },
            4017108033 => match prop {
                _ => 0,
            },
            4021432810 => match prop {
                _ => 0,
            },
            4022376103 => match prop {
                _ => 0,
            },
            4024345920 => match prop {
                _ => 0,
            },
            4031249490 => match prop {
                _ => 0,
            },
            4037036970 => match prop {
                _ => 0,
            },
            4037862832 => match prop {
                _ => 0,
            },
            4074379575 => match prop {
                _ => 0,
            },
            4074543187 => match prop {
                _ => 0,
            },
            4086658281 => match prop {
                _ => 0,
            },
            4088093105 => match prop {
                _ => 0,
            },
            4095422895 => match prop {
                _ => 0,
            },
            4095574036 => match prop {
                _ => 0,
            },
            4095615324 => match prop {
                _ => 0,
            },
            4097777520 => match prop {
                _ => 0,
            },
            4105526436 => match prop {
                _ => 0,
            },
            4105962743 => match prop {
                _ => 0,
            },
            4122056220 => match prop {
                _ => 0,
            },
            4123344466 => match prop {
                _ => 0,
            },
            4124623270 => match prop {
                _ => 0,
            },
            4124788165 => match prop {
                _ => 0,
            },
            4136498852 => match prop {
                _ => 0,
            },
            4142052618 => match prop {
                _ => 0,
            },
            4143007308 => match prop {
                _ => 0,
            },
            4148101412 => match prop {
                _ => 0,
            },
            4158566097 => match prop {
                _ => 0,
            },
            4162380809 => match prop {
                _ => 0,
            },
            4165799628 => match prop {
                _ => 0,
            },
            4166981789 => match prop {
                _ => 0,
            },
            4175244083 => match prop {
                _ => 0,
            },
            4182860854 => match prop {
                _ => 0,
            },
            4186316022 => match prop {
                _ => 0,
            },
            4189326743 => match prop {
                _ => 0,
            },
            4196446775 => match prop {
                _ => 0,
            },
            4201705270 => match prop {
                _ => 0,
            },
            4207607924 => match prop {
                _ => 0,
            },
            4208778838 => match prop {
                _ => 0,
            },
            4212018352 => match prop {
                _ => 0,
            },
            4217484030 => match prop {
                _ => 0,
            },
            4218914973 => match prop {
                _ => 0,
            },
            4219587988 => match prop {
                _ => 0,
            },
            4222183408 => match prop {
                _ => 0,
            },
            4228831410 => match prop {
                _ => 0,
            },
            4230923436 => match prop {
                _ => 0,
            },
            4231323485 => match prop {
                _ => 0,
            },
            4234616927 => match prop {
                _ => 0,
            },
            4237592921 => match prop {
                _ => 0,
            },
            4238390223 => match prop {
                _ => 0,
            },
            4240577450 => match prop {
                _ => 0,
            },
            4243806635 => match prop {
                _ => 0,
            },
            4251960020 => match prop {
                _ => 0,
            },
            4252922144 => match prop {
                _ => 0,
            },
            4261334040 => match prop {
                _ => 0,
            },
            4266260250 => match prop {
                _ => 0,
            },
            4266656042 => match prop {
                _ => 0,
            },
            4278684876 => match prop {
                _ => 0,
            },
            4278956645 => match prop {
                _ => 0,
            },
            4282788508 => match prop {
                _ => 0,
            },
            4288193352 => match prop {
                _ => 0,
            },
            4288270099 => match prop {
                _ => 0,
            },
            4292641817 => match prop {
                _ => 0,
            },
            4294318154 => match prop {
                _ => 0,
            },
            _ => 0,
        },
    }
}

#[inline]
pub fn get_property_count(schema: IFC_SCHEMA, type_code: u32) -> u32 {
    match schema {
        IFC_SCHEMA::IFC2X3 => match type_code {
            5716631 => 10,
            30780891 => 5,
            32440307 => 1,
            45288368 => 5,
            52481810 => 8,
            59481748 => 4,
            80994333 => 6,
            101040310 => 3,
            103090709 => 9,
            110355661 => 7,
            125510826 => 4,
            130549933 => 7,
            148013059 => 11,
            148025276 => 5,
            160246688 => 6,
            178086475 => 2,
            179317114 => 1,
            180925521 => 1,
            194851669 => 12,
            200128114 => 10,
            202636808 => 7,
            205026976 => 7,
            214636428 => 8,
            219451334 => 4,
            220341763 => 1,
            230924584 => 2,
            231477066 => 10,
            248100487 => 3,
            263784265 => 8,
            279856033 => 6,
            280115917 => 0,
            300633059 => 10,
            315944413 => 3,
            331165859 => 9,
            335055490 => 10,
            336235671 => 13,
            339256511 => 9,
            346874300 => 10,
            347226245 => 4,
            360485395 => 14,
            366585022 => 6,
            370225590 => 1,
            374418227 => 5,
            377706215 => 10,
            390701378 => 14,
            390851274 => 2,
            395041908 => 10,
            395920057 => 10,
            411424972 => 6,
            427810014 => 11,
            433424934 => 1,
            445594917 => 1,
            448429030 => 4,
            451544542 => 2,
            476780140 => 4,
            477187591 => 4,
            478536968 => 4,
            488727124 => 9,
            504942748 => 11,
            512836454 => 9,
            526551008 => 12,
            530289379 => 7,
            531007025 => 2,
            539742890 => 5,
            572779678 => 11,
            578613899 => 10,
            581633288 => 1,
            597895409 => 8,
            602808272 => 8,
            603570806 => 3,
            606661476 => 3,
            613356794 => 2,
            616511568 => 6,
            618182010 => 3,
            622194075 => 3,
            626085974 => 4,
            639542469 => 4,
            647756555 => 8,
            647927063 => 4,
            652456506 => 10,
            663422040 => 10,
            669184980 => 2,
            673634403 => 3,
            677618848 => 13,
            681481545 => 1,
            682877961 => 11,
            693640335 => 5,
            693772133 => 2,
            707683696 => 8,
            712377611 => 10,
            723233188 => 0,
            728799441 => 10,
            734778138 => 8,
            738692330 => 2,
            747523909 => 4,
            750771296 => 6,
            753842376 => 8,
            759155922 => 1,
            770865208 => 3,
            776857604 => 4,
            781010003 => 6,
            803316827 => 2,
            803998398 => 4,
            804291784 => 10,
            807026263 => 1,
            812098782 => 2,
            814719939 => 5,
            819618141 => 10,
            825690147 => 4,
            826625072 => 4,
            843113511 => 8,
            846575682 => 1,
            852622518 => 3,
            855621170 => 9,
            857184966 => 8,
            867548509 => 5,
            869906466 => 10,
            871118103 => 5,
            886880790 => 6,
            891718957 => 2,
            900683007 => 9,
            912023232 => 8,
            919958153 => 6,
            931644368 => 4,
            938368621 => 1,
            941946838 => 4,
            962685235 => 3,
            977012517 => 10,
            979691226 => 14,
            982818633 => 6,
            985171141 => 2,
            987401354 => 8,
            987898635 => 1,
            990879717 => 1,
            1008929658 => 0,
            1028945134 => 15,
            1029017970 => 4,
            1033361043 => 5,
            1039846685 => 10,
            1040185647 => 3,
            1045800335 => 2,
            1051575348 => 10,
            1052013943 => 8,
            1058617721 => 7,
            1060000209 => 11,
            1062813311 => 9,
            1065062679 => 3,
            1065908215 => 8,
            1072939445 => 2,
            1073191201 => 8,
            1076942058 => 4,
            1095909175 => 9,
            1098599126 => 2,
            1105321065 => 2,
            1110488051 => 5,
            1123145078 => 1,
            1133259667 => 10,
            1154170062 => 17,
            1161773419 => 10,
            1163958913 => 7,
            1179482911 => 8,
            1190533807 => 8,
            1202362311 => 7,
            1204542856 => 7,
            1207048766 => 8,
            1210645708 => 1,
            1213861670 => 2,
            1217240411 => 10,
            1222501353 => 2,
            1227763645 => 10,
            1235345126 => 9,
            1245217292 => 6,
            1251058090 => 10,
            1252848954 => 10,
            1260505505 => 0,
            1260650574 => 5,
            1268542332 => 10,
            1281925730 => 2,
            1285652485 => 10,
            1287392070 => 8,
            1290481447 => 2,
            1299126871 => 12,
            1300840506 => 3,
            1302238472 => 2,
            1303795690 => 4,
            1304840413 => 8,
            1305183839 => 10,
            1307041759 => 7,
            1310608509 => 3,
            1327628568 => 6,
            1334484129 => 4,
            1335981549 => 8,
            1339347760 => 9,
            1345879162 => 2,
            1351298697 => 1,
            1365060375 => 10,
            1376555844 => 3,
            1376911519 => 10,
            1377556343 => 0,
            1383045692 => 4,
            1387855156 => 7,
            1401173127 => 6,
            1402838566 => 4,
            1411181986 => 4,
            1411407467 => 10,
            1416205885 => 7,
            1417489154 => 2,
            1419761937 => 13,
            1423911732 => 3,
            1425443689 => 1,
            1430189142 => 12,
            1446786286 => 7,
            1447204868 => 4,
            1451395588 => 5,
            1457835157 => 10,
            1472233963 => 1,
            1482959167 => 9,
            1484403080 => 8,
            1484833681 => 5,
            1485152156 => 4,
            1509187699 => 3,
            1520743889 => 9,
            1529196076 => 9,
            1534661035 => 10,
            1560379544 => 7,
            1566485204 => 2,
            1580146022 => 6,
            1580310250 => 9,
            1595516126 => 7,
            1597423693 => 7,
            1599208980 => 10,
            1600972822 => 10,
            1607154358 => 2,
            1620046519 => 11,
            1621171031 => 12,
            1623761950 => 8,
            1628702193 => 6,
            1634875225 => 5,
            1637806684 => 8,
            1638771189 => 10,
            1640371178 => 7,
            1648886627 => 9,
            1658513725 => 5,
            1658829314 => 8,
            1660063152 => 2,
            1663979128 => 2,
            1674181508 => 7,
            1680319473 => 4,
            1683148259 => 8,
            1687234759 => 10,
            1692211062 => 10,
            1694125774 => 4,
            1704287377 => 3,
            1714330368 => 9,
            1718945513 => 2,
            1721250024 => 14,
            1735638870 => 4,
            1742049831 => 2,
            1758889154 => 8,
            1765591967 => 3,
            1767535486 => 3,
            1768891740 => 10,
            1775413392 => 1,
            1783015770 => 10,
            1806887404 => 10,
            1807405624 => 12,
            1809719519 => 2,
            1810631287 => 10,
            1834744321 => 9,
            1838606355 => 1,
            1842657554 => 10,
            1847130766 => 2,
            1856042241 => 4,
            1860660968 => 4,
            1865459582 => 5,
            1871374353 => 10,
            1878645084 => 9,
            1883228015 => 6,
            1898987631 => 10,
            1907098498 => 1,
            1909888760 => 10,
            1911125066 => 10,
            1916426348 => 10,
            1916936684 => 13,
            1916977116 => 5,
            1918398963 => 2,
            1945004755 => 8,
            1950629157 => 9,
            1959218052 => 7,
            1962604670 => 8,
            1967976161 => 5,
            1973038258 => 8,
            1973544240 => 9,
            1975003073 => 8,
            1981873012 => 2,
            1983826977 => 6,
            2004835150 => 1,
            2016517767 => 9,
            2022407955 => 4,
            2022622350 => 4,
            2028607225 => 6,
            2030761528 => 10,
            2042790032 => 3,
            2044713172 => 4,
            2047409740 => 1,
            2051452291 => 8,
            2058353004 => 8,
            2063403501 => 9,
            2067069095 => 0,
            2069777674 => 8,
            2077209135 => 8,
            2080292479 => 3,
            2082059205 => 11,
            2093928680 => 4,
            2095639259 => 3,
            2097647324 => 10,
            2107101300 => 9,
            2108223431 => 9,
            2127690289 => 6,
            2143335405 => 8,
            2147822146 => 1,
            2162789131 => 1,
            2188551683 => 5,
            2199411900 => 1,
            2205249479 => 1,
            2218152070 => 11,
            2223149337 => 8,
            2226359599 => 3,
            2233826070 => 3,
            2242383968 => 3,
            2247615214 => 2,
            2250791053 => 10,
            2251480897 => 11,
            2254336722 => 5,
            2262370178 => 9,
            2265737646 => 5,
            2267347899 => 5,
            2273265877 => 4,
            2273995522 => 1,
            2296667514 => 6,
            2297155007 => 9,
            2297822566 => 3,
            2301859152 => 10,
            2315554128 => 10,
            2320036040 => 17,
            2324767716 => 10,
            2341007311 => 4,
            2347385850 => 2,
            2347447852 => 9,
            2347495698 => 8,
            2367409068 => 3,
            2382730787 => 6,
            2391368822 => 11,
            2391406946 => 8,
            2405470396 => 4,
            2411513650 => 8,
            2417041796 => 1,
            2442683028 => 3,
            2445078500 => 6,
            2445595289 => 8,
            2453401579 => 0,
            2454782716 => 11,
            2470393545 => 1,
            2473145415 => 7,
            2483315170 => 2,
            2485617015 => 3,
            2485662743 => 7,
            2489546625 => 9,
            2495723537 => 7,
            2506170314 => 1,
            2506943328 => 1,
            2510884976 => 1,
            2513912981 => 0,
            2515109513 => 9,
            2519244187 => 1,
            2525727697 => 1,
            2529465313 => 3,
            2533589738 => 10,
            2542286263 => 4,
            2543172580 => 9,
            2551354335 => 6,
            2552916305 => 1,
            2556980723 => 1,
            2559016684 => 1,
            2559216714 => 9,
            2581212453 => 4,
            2590856083 => 9,
            2597039031 => 2,
            2598011224 => 2,
            2601014836 => 0,
            2604431987 => 5,
            2609359061 => 4,
            2611217952 => 2,
            2614616156 => 2,
            2624227202 => 2,
            2635815018 => 9,
            2636378356 => 2,
            2655187982 => 5,
            2655215786 => 6,
            2665983363 => 1,
            2668620305 => 4,
            2692823254 => 4,
            2705031697 => 4,
            2706460486 => 5,
            2706606064 => 9,
            2706619895 => 1,
            2713105998 => 3,
            2715220739 => 7,
            2728634034 => 7,
            2732653382 => 2,
            2736907675 => 3,
            2740243338 => 3,
            2744685151 => 8,
            2752243245 => 4,
            2759199220 => 1,
            2769231204 => 8,
            2770003689 => 8,
            2775532180 => 4,
            2777663545 => 1,
            2778083089 => 6,
            2798486643 => 4,
            2799835756 => 0,
            2802773753 => 6,
            2802850158 => 2,
            2809605785 => 4,
            2816379211 => 10,
            2827207264 => 8,
            2827736869 => 3,
            2830218821 => 4,
            2833995503 => 1,
            2835456948 => 5,
            2837617999 => 10,
            2851387026 => 8,
            2857406711 => 7,
            2859738748 => 0,
            2863920197 => 8,
            2874132201 => 10,
            2889183280 => 4,
            2893384427 => 10,
            2898889636 => 9,
            2904328755 => 8,
            2914609552 => 5,
            2924175390 => 1,
            2937912522 => 5,
            2945172077 => 5,
            2949456006 => 7,
            2951183804 => 10,
            2954562838 => 10,
            2963535650 => 15,
            2979338954 => 8,
            2986769608 => 8,
            3001207471 => 10,
            3008276851 => 3,
            3008791417 => 0,
            3009204131 => 10,
            3009222698 => 9,
            3020489413 => 2,
            3021840470 => 6,
            3024970846 => 9,
            3027567501 => 9,
            3028897424 => 4,
            3040386961 => 8,
            3041715199 => 8,
            3049322572 => 4,
            3050246964 => 3,
            3055160366 => 6,
            3071757647 => 13,
            3073041342 => 1,
            3101149627 => 8,
            3112655638 => 10,
            3119450353 => 1,
            3124254112 => 10,
            3124975700 => 5,
            3125803723 => 2,
            3132237377 => 8,
            3136571912 => 7,
            3150382593 => 4,
            3171933400 => 8,
            3174744832 => 10,
            3181161470 => 10,
            3190031847 => 7,
            3198132628 => 9,
            3200245327 => 3,
            3207319532 => 3,
            3207858831 => 12,
            3213052703 => 1,
            3219374653 => 9,
            3242617779 => 6,
            3248260540 => 1,
            3252649465 => 4,
            3256556792 => 9,
            3264961684 => 1,
            3265635763 => 1,
            3268803585 => 6,
            3272907226 => 5,
            3277789161 => 10,
            3283111854 => 8,
            3288037868 => 3,
            3293443760 => 5,
            3293546465 => 10,
            3295246426 => 9,
            3299480353 => 8,
            3303107099 => 4,
            3303938423 => 2,
            3304561284 => 10,
            3304826586 => 2,
            3313531582 => 10,
            3317419933 => 5,
            3327091369 => 6,
            3331915920 => 5,
            3342526732 => 15,
            3352864051 => 10,
            3355820592 => 10,
            3357820518 => 4,
            3367102660 => 4,
            3368373690 => 10,
            3372526763 => 7,
            3377609919 => 2,
            3388369263 => 3,
            3390157468 => 10,
            3408363356 => 4,
            3413951693 => 10,
            3422422726 => 13,
            3425660407 => 11,
            3448662350 => 6,
            3451746338 => 9,
            3452421091 => 3,
            3454111270 => 7,
            3455213021 => 19,
            3460190687 => 14,
            3473067441 => 10,
            3486308946 => 5,
            3495092785 => 8,
            3505215534 => 4,
            3508470533 => 8,
            3510044353 => 2,
            3512223829 => 8,
            3517283431 => 23,
            3544373492 => 9,
            3548104201 => 3,
            3566463478 => 9,
            3588315303 => 8,
            3590301190 => 1,
            3593883385 => 5,
            3612888222 => 3,
            3615266464 => 5,
            3626867408 => 3,
            3630933823 => 3,
            3632507154 => 5,
            3639012971 => 1,
            3642467123 => 7,
            3649129432 => 3,
            3650150729 => 4,
            3651124850 => 8,
            3653947884 => 27,
            3678494232 => 9,
            3679540991 => 7,
            3689010777 => 9,
            3692461612 => 2,
            3700593921 => 10,
            3701648758 => 0,
            3710013099 => 3,
            3724593414 => 1,
            3727388367 => 1,
            3732053477 => 3,
            3732776249 => 2,
            3737207727 => 2,
            3740093272 => 7,
            3741457305 => 9,
            3749851601 => 4,
            3760055223 => 10,
            3765753017 => 6,
            3796139169 => 4,
            3798115385 => 3,
            3800577675 => 4,
            3812236995 => 10,
            3815607619 => 10,
            3821786052 => 6,
            3824725483 => 17,
            3827777499 => 10,
            3840914261 => 6,
            3843319758 => 23,
            3849074793 => 9,
            3850581409 => 10,
            3856911033 => 11,
            3857492461 => 5,
            3869604511 => 4,
            3888040117 => 5,
            3893378262 => 9,
            3895139033 => 5,
            3896028662 => 4,
            3898045240 => 9,
            3900360178 => 2,
            3905492369 => 5,
            3907093117 => 9,
            3912681535 => 6,
            3939117080 => 6,
            3940055652 => 6,
            3945020480 => 11,
            3958052878 => 3,
            3958567839 => 2,
            3961806047 => 10,
            3979015343 => 9,
            3982875396 => 4,
            3987759626 => 14,
            4006246654 => 1,
            4017108033 => 10,
            4022376103 => 2,
            4031249490 => 12,
            4037036970 => 1,
            4037862832 => 10,
            4054601972 => 5,
            4070609034 => 1,
            4095574036 => 6,
            4097777520 => 14,
            4105383287 => 7,
            4122056220 => 8,
            4123344466 => 10,
            4124623270 => 1,
            4124788165 => 3,
            4133800736 => 15,
            4142052618 => 10,
            4143007308 => 7,
            4147604152 => 1,
            4158566097 => 3,
            4162380809 => 3,
            4165799628 => 6,
            4166981789 => 4,
            4170525392 => 1,
            4182860854 => 0,
            4186316022 => 6,
            4189434867 => 9,
            4194566429 => 3,
            4201705270 => 6,
            4203026998 => 1,
            4208778838 => 7,
            4218914973 => 15,
            4219587988 => 7,
            4222183408 => 10,
            4231323485 => 10,
            4238390223 => 9,
            4240577450 => 4,
            4243806635 => 8,
            4251960020 => 5,
            4252922144 => 12,
            4256014907 => 6,
            4257277454 => 3,
            4261334040 => 2,
            4266656042 => 10,
            4278684876 => 8,
            4278956645 => 8,
            4282788508 => 3,
            4288270099 => 10,
            _ => 0,
        },
        IFC_SCHEMA::IFC4 => match type_code {
            5716631 => 10,
            15328376 => 2,
            25142252 => 9,
            32344328 => 9,
            32440307 => 1,
            39481116 => 10,
            45288368 => 5,
            59481748 => 4,
            76236018 => 9,
            90941305 => 9,
            101040310 => 3,
            103090709 => 9,
            110355661 => 8,
            125510826 => 4,
            130549933 => 9,
            132023988 => 10,
            144952367 => 2,
            148013059 => 11,
            148025276 => 5,
            160246688 => 6,
            164193824 => 4,
            167062518 => 12,
            177149247 => 9,
            178086475 => 2,
            178912537 => 1,
            180925521 => 1,
            182646315 => 9,
            205026976 => 7,
            211053100 => 7,
            214636428 => 9,
            219451334 => 4,
            220341763 => 1,
            230924584 => 2,
            231477066 => 10,
            248100487 => 7,
            263784265 => 8,
            264262732 => 9,
            277319702 => 9,
            279856033 => 6,
            280115917 => 1,
            297599258 => 3,
            300633059 => 10,
            307848117 => 6,
            310824031 => 9,
            315944413 => 3,
            331165859 => 9,
            335055490 => 10,
            336235671 => 16,
            338393293 => 10,
            339256511 => 9,
            342316401 => 9,
            346874300 => 10,
            366585022 => 6,
            370225590 => 1,
            374418227 => 5,
            377706215 => 11,
            385403989 => 11,
            395041908 => 10,
            395920057 => 13,
            400855858 => 10,
            402227799 => 9,
            411424972 => 10,
            413509423 => 9,
            427810014 => 10,
            427948657 => 9,
            428585644 => 12,
            445594917 => 1,
            448429030 => 4,
            451544542 => 2,
            463610769 => 9,
            476780140 => 4,
            477187591 => 4,
            478536968 => 4,
            484807127 => 9,
            486154966 => 13,
            488727124 => 11,
            492091185 => 7,
            504942748 => 11,
            512836454 => 9,
            526551008 => 12,
            530289379 => 7,
            531007025 => 2,
            539742890 => 7,
            552965576 => 7,
            562808652 => 7,
            569719735 => 10,
            572779678 => 9,
            574549367 => 0,
            578613899 => 10,
            581633288 => 1,
            597895409 => 9,
            602808272 => 10,
            603570806 => 3,
            603775116 => 10,
            609421318 => 1,
            616511568 => 7,
            618182010 => 3,
            626085974 => 5,
            629592764 => 9,
            630975310 => 9,
            635142910 => 9,
            639361253 => 9,
            639542469 => 4,
            647756555 => 9,
            647927063 => 6,
            653396225 => 9,
            655969474 => 10,
            663422040 => 10,
            669184980 => 2,
            673634403 => 3,
            677532197 => 0,
            682877961 => 10,
            683857671 => 13,
            693640335 => 4,
            699246055 => 3,
            707683696 => 8,
            710998568 => 9,
            712377611 => 10,
            723233188 => 0,
            728799441 => 10,
            734778138 => 9,
            738039164 => 9,
            738692330 => 3,
            747523909 => 7,
            750771296 => 6,
            753842376 => 9,
            759155922 => 1,
            760658860 => 0,
            770865208 => 5,
            775493141 => 2,
            776857604 => 4,
            781010003 => 6,
            803316827 => 2,
            804291784 => 10,
            807026263 => 1,
            812098782 => 2,
            812556717 => 9,
            816062949 => 4,
            819412036 => 9,
            819618141 => 10,
            825690147 => 5,
            826625072 => 4,
            843113511 => 9,
            846575682 => 2,
            852622518 => 3,
            853536259 => 5,
            862014818 => 9,
            867548509 => 5,
            869906466 => 10,
            871118103 => 6,
            886880790 => 6,
            891718957 => 2,
            900683007 => 9,
            901063453 => 0,
            905975707 => 9,
            912023232 => 9,
            919958153 => 6,
            926996030 => 9,
            931644368 => 5,
            941946838 => 4,
            964333572 => 9,
            977012517 => 10,
            979691226 => 14,
            982818633 => 6,
            985171141 => 3,
            986844984 => 0,
            987401354 => 8,
            987898635 => 1,
            1003880860 => 9,
            1004757350 => 12,
            1008929658 => 0,
            1027710054 => 8,
            1028945134 => 13,
            1029017970 => 4,
            1033361043 => 6,
            1039846685 => 10,
            1040185647 => 3,
            1042787934 => 18,
            1045800335 => 2,
            1051575348 => 10,
            1051757585 => 9,
            1052013943 => 9,
            1054537805 => 3,
            1060000209 => 11,
            1062813311 => 8,
            1072016465 => 10,
            1073191201 => 9,
            1076942058 => 4,
            1095909175 => 9,
            1096409881 => 6,
            1105321065 => 2,
            1114901282 => 10,
            1123145078 => 1,
            1133259667 => 10,
            1136057603 => 2,
            1154170062 => 17,
            1156407060 => 9,
            1158309216 => 10,
            1161773419 => 10,
            1162798199 => 9,
            1177604601 => 7,
            1179482911 => 8,
            1190533807 => 8,
            1199560280 => 2,
            1204542856 => 7,
            1207048766 => 8,
            1209101575 => 9,
            1210645708 => 1,
            1213902940 => 2,
            1217240411 => 10,
            1232101972 => 9,
            1235345126 => 9,
            1236880293 => 6,
            1245217292 => 6,
            1251058090 => 10,
            1252848954 => 10,
            1260505505 => 0,
            1260650574 => 5,
            1268542332 => 11,
            1281925730 => 2,
            1285652485 => 10,
            1287392070 => 8,
            1299126871 => 12,
            1300840506 => 3,
            1303795690 => 5,
            1304840413 => 8,
            1305183839 => 10,
            1307041759 => 7,
            1310608509 => 3,
            1329646415 => 9,
            1334484129 => 4,
            1335981549 => 9,
            1339347760 => 9,
            1351298697 => 1,
            1360408905 => 9,
            1377556343 => 0,
            1383045692 => 4,
            1387855156 => 7,
            1401173127 => 6,
            1402838566 => 4,
            1404847402 => 9,
            1411181986 => 4,
            1411407467 => 10,
            1412071761 => 8,
            1416205885 => 7,
            1417489154 => 2,
            1419761937 => 10,
            1423911732 => 3,
            1425443689 => 1,
            1426591983 => 9,
            1437502449 => 9,
            1437805879 => 4,
            1437953363 => 3,
            1447204868 => 5,
            1451395588 => 5,
            1457835157 => 10,
            1462361463 => 6,
            1466758467 => 4,
            1469900589 => 10,
            1472233963 => 1,
            1482703590 => 4,
            1482959167 => 9,
            1484403080 => 10,
            1485152156 => 4,
            1507914824 => 0,
            1509187699 => 3,
            1509553395 => 9,
            1520743889 => 9,
            1521410863 => 11,
            1525564444 => 12,
            1529196076 => 9,
            1532957894 => 10,
            1534661035 => 10,
            1549132990 => 20,
            1560379544 => 7,
            1566485204 => 2,
            1580146022 => 6,
            1580310250 => 10,
            1585845231 => 5,
            1595516126 => 7,
            1597423693 => 7,
            1599208980 => 10,
            1600972822 => 10,
            1607154358 => 2,
            1608871552 => 4,
            1620046519 => 9,
            1621171031 => 12,
            1623761950 => 8,
            1628702193 => 6,
            1634111441 => 9,
            1635779807 => 1,
            1638771189 => 10,
            1640371178 => 7,
            1658829314 => 8,
            1660063152 => 2,
            1663979128 => 2,
            1674181508 => 7,
            1675464909 => 1,
            1677625105 => 8,
            1680319473 => 4,
            1682466193 => 2,
            1683148259 => 8,
            1687234759 => 10,
            1692211062 => 10,
            1704287377 => 3,
            1714330368 => 9,
            1735638870 => 4,
            1742049831 => 3,
            1758889154 => 8,
            1765591967 => 3,
            1768891740 => 10,
            1775413392 => 1,
            1783015770 => 10,
            1785450214 => 2,
            1806887404 => 10,
            1807405624 => 12,
            1809719519 => 2,
            1810631287 => 10,
            1815067380 => 12,
            1834744321 => 9,
            1838606355 => 3,
            1842657554 => 10,
            1847130766 => 2,
            1847252529 => 9,
            1856042241 => 4,
            1865459582 => 5,
            1871374353 => 10,
            1878645084 => 9,
            1883228015 => 6,
            1893162501 => 10,
            1898987631 => 10,
            1904799276 => 9,
            1907098498 => 1,
            1909888760 => 10,
            1911125066 => 10,
            1911478936 => 9,
            1916426348 => 10,
            1918398963 => 2,
            1935646853 => 3,
            1945004755 => 8,
            1950629157 => 9,
            1959218052 => 7,
            1967976161 => 5,
            1973038258 => 8,
            1973544240 => 9,
            1975003073 => 8,
            1981873012 => 2,
            1983826977 => 6,
            1999602285 => 9,
            2004835150 => 1,
            2016517767 => 9,
            2022407955 => 4,
            2022622350 => 4,
            2028607225 => 6,
            2030761528 => 10,
            2042790032 => 3,
            2043862942 => 5,
            2044713172 => 5,
            2047409740 => 1,
            2056796094 => 9,
            2058353004 => 8,
            2059837836 => 1,
            2063403501 => 9,
            2067069095 => 0,
            2068733104 => 9,
            2069777674 => 8,
            2077209135 => 8,
            2082059205 => 10,
            2090586900 => 4,
            2093928680 => 5,
            2095639259 => 3,
            2097647324 => 10,
            2107101300 => 9,
            2108223431 => 12,
            2127690289 => 6,
            2133299955 => 4,
            2143335405 => 8,
            2147822146 => 1,
            2157484638 => 3,
            2162789131 => 1,
            2176052936 => 9,
            2185764099 => 12,
            2188021234 => 9,
            2188180465 => 10,
            2197970202 => 10,
            2199411900 => 1,
            2205249479 => 1,
            2218152070 => 9,
            2223149337 => 8,
            2226359599 => 3,
            2233826070 => 3,
            2235152071 => 6,
            2242383968 => 3,
            2247615214 => 2,
            2250791053 => 10,
            2251480897 => 11,
            2254336722 => 5,
            2262370178 => 9,
            2272882330 => 9,
            2273995522 => 1,
            2294589976 => 2,
            2295281155 => 9,
            2296667514 => 6,
            2297155007 => 9,
            2301859152 => 10,
            2310774935 => 20,
            2315554128 => 10,
            2320036040 => 18,
            2323601079 => 13,
            2324767716 => 10,
            2341007311 => 4,
            2347385850 => 2,
            2347447852 => 10,
            2347495698 => 8,
            2367409068 => 3,
            2382730787 => 8,
            2387106220 => 1,
            2391368822 => 11,
            2391383451 => 9,
            2391406946 => 9,
            2397081782 => 10,
            2405470396 => 5,
            2415094496 => 13,
            2417008758 => 10,
            2417041796 => 1,
            2433181523 => 5,
            2439245199 => 2,
            2445595289 => 9,
            2453401579 => 0,
            2461110595 => 8,
            2473145415 => 7,
            2474470126 => 9,
            2481509218 => 11,
            2483315170 => 2,
            2485617015 => 3,
            2489546625 => 10,
            2495723537 => 7,
            2506170314 => 1,
            2510884976 => 1,
            2513912981 => 0,
            2515109513 => 10,
            2519244187 => 1,
            2525727697 => 1,
            2529465313 => 3,
            2533589738 => 10,
            2542286263 => 4,
            2543172580 => 9,
            2551354335 => 4,
            2552916305 => 3,
            2556980723 => 1,
            2559016684 => 1,
            2559216714 => 10,
            2565941209 => 6,
            2571569899 => 3,
            2572171363 => 16,
            2574617495 => 11,
            2581212453 => 4,
            2590856083 => 9,
            2597039031 => 2,
            2598011224 => 2,
            2601014836 => 0,
            2603310189 => 2,
            2604431987 => 5,
            2609359061 => 4,
            2611217952 => 2,
            2614616156 => 2,
            2624227202 => 2,
            2629017746 => 3,
            2635815018 => 10,
            2636378356 => 2,
            2652556860 => 6,
            2655187982 => 6,
            2655215786 => 6,
            2665983363 => 1,
            2668620305 => 4,
            2674252688 => 10,
            2705031697 => 4,
            2706460486 => 5,
            2706606064 => 9,
            2706619895 => 1,
            2713105998 => 3,
            2713554722 => 5,
            2715220739 => 7,
            2728634034 => 7,
            2732653382 => 2,
            2736907675 => 3,
            2740243338 => 3,
            2744685151 => 8,
            2752243245 => 4,
            2757150158 => 10,
            2759199220 => 1,
            2769231204 => 8,
            2770003689 => 8,
            2771591690 => 21,
            2775532180 => 4,
            2777663545 => 1,
            2778083089 => 6,
            2781568857 => 10,
            2798486643 => 4,
            2799835756 => 0,
            2802773753 => 6,
            2802850158 => 4,
            2804161546 => 5,
            2809605785 => 4,
            2814081492 => 9,
            2816379211 => 10,
            2827207264 => 8,
            2827736869 => 3,
            2830218821 => 4,
            2835456948 => 5,
            2837617999 => 10,
            2839578677 => 4,
            2852063980 => 3,
            2853485674 => 8,
            2857406711 => 7,
            2859738748 => 0,
            2874132201 => 10,
            2887950389 => 7,
            2889183280 => 4,
            2893384427 => 10,
            2898889636 => 8,
            2904328755 => 9,
            2906023776 => 9,
            2914609552 => 7,
            2916149573 => 5,
            2924175390 => 1,
            2934153892 => 4,
            2937912522 => 5,
            2938176219 => 9,
            2943643501 => 4,
            2945172077 => 7,
            2949456006 => 7,
            2951183804 => 10,
            2954562838 => 10,
            2963535650 => 17,
            2979338954 => 9,
            2986769608 => 8,
            2998442950 => 5,
            3001207471 => 10,
            3008276851 => 3,
            3008791417 => 0,
            3009204131 => 11,
            3009222698 => 9,
            3020489413 => 2,
            3021840470 => 6,
            3024970846 => 9,
            3026737570 => 9,
            3027567501 => 9,
            3027962421 => 9,
            3040386961 => 8,
            3041715199 => 10,
            3049322572 => 4,
            3050246964 => 3,
            3053780830 => 9,
            3057273783 => 8,
            3071757647 => 12,
            3079605661 => 3,
            3079942009 => 9,
            3081323446 => 10,
            3087945054 => 9,
            3101149627 => 8,
            3101698114 => 9,
            3112655638 => 10,
            3113134337 => 3,
            3119450353 => 1,
            3124254112 => 10,
            3124975700 => 5,
            3125803723 => 2,
            3127900445 => 9,
            3132237377 => 8,
            3136571912 => 7,
            3150382593 => 4,
            3171933400 => 9,
            3174744832 => 10,
            3179687236 => 10,
            3181161470 => 10,
            3190031847 => 7,
            3198132628 => 9,
            3200245327 => 3,
            3205830791 => 7,
            3206491090 => 11,
            3207858831 => 15,
            3219374653 => 9,
            3221913625 => 9,
            3242481149 => 13,
            3242617779 => 6,
            3243963512 => 5,
            3252649465 => 5,
            3256556792 => 9,
            3264961684 => 1,
            3265635763 => 4,
            3268803585 => 6,
            3277789161 => 10,
            3283111854 => 9,
            3285139300 => 1,
            3293443760 => 6,
            3293546465 => 10,
            3295246426 => 11,
            3296154744 => 9,
            3299480353 => 8,
            3303107099 => 4,
            3303938423 => 3,
            3304561284 => 13,
            3310460725 => 9,
            3313531582 => 10,
            3319311131 => 9,
            3327091369 => 9,
            3331915920 => 5,
            3342526732 => 14,
            3352864051 => 10,
            3355820592 => 10,
            3357820518 => 4,
            3367102660 => 4,
            3368373690 => 11,
            3377609919 => 2,
            3388369263 => 3,
            3390157468 => 10,
            3404854881 => 5,
            3406155212 => 3,
            3408363356 => 4,
            3413951693 => 10,
            3415622556 => 9,
            3419103109 => 9,
            3420628829 => 9,
            3422422726 => 13,
            3448662350 => 6,
            3451746338 => 9,
            3452421091 => 6,
            3454111270 => 7,
            3460190687 => 14,
            3473067441 => 13,
            3478079324 => 3,
            3486308946 => 5,
            3493046030 => 9,
            3495092785 => 9,
            3505215534 => 4,
            3508470533 => 8,
            3510044353 => 2,
            3512223829 => 9,
            3518393246 => 9,
            3521284610 => 4,
            3523091289 => 10,
            3544373492 => 9,
            3548104201 => 3,
            3566463478 => 9,
            3570813810 => 4,
            3571504051 => 9,
            3588315303 => 9,
            3590301190 => 1,
            3593883385 => 5,
            3611470254 => 1,
            3612865200 => 9,
            3615266464 => 5,
            3626867408 => 3,
            3630933823 => 3,
            3632507154 => 5,
            3640358203 => 9,
            3649129432 => 3,
            3650150729 => 4,
            3651124850 => 9,
            3657597509 => 12,
            3663146110 => 12,
            3678494232 => 9,
            3689010777 => 9,
            3692461612 => 2,
            3694346114 => 9,
            3698973494 => 9,
            3701648758 => 0,
            3708119000 => 5,
            3710013099 => 3,
            3724593414 => 1,
            3727388367 => 1,
            3732053477 => 5,
            3732776249 => 2,
            3736923433 => 9,
            3737207727 => 2,
            3740093272 => 7,
            3741457305 => 9,
            3747195512 => 9,
            3749851601 => 4,
            3758799889 => 9,
            3760055223 => 10,
            3765753017 => 6,
            3778827333 => 0,
            3798115385 => 3,
            3800577675 => 5,
            3812236995 => 11,
            3815607619 => 10,
            3821786052 => 9,
            3824725483 => 17,
            3825984169 => 9,
            3827777499 => 11,
            3840914261 => 6,
            3843373140 => 7,
            3849074793 => 9,
            3850581409 => 10,
            3856911033 => 11,
            3869604511 => 4,
            3875453745 => 7,
            3888040117 => 5,
            3893378262 => 9,
            3893394355 => 9,
            3895139033 => 9,
            3898045240 => 11,
            3900360178 => 2,
            3902619387 => 9,
            3905492369 => 6,
            3907093117 => 9,
            3915482550 => 8,
            3939117080 => 6,
            3940055652 => 6,
            3945020480 => 11,
            3946677679 => 10,
            3958052878 => 3,
            3958567839 => 2,
            3961806047 => 10,
            3967405729 => 4,
            3979015343 => 9,
            3982875396 => 4,
            4006246654 => 1,
            4009809668 => 13,
            4015995234 => 2,
            4017108033 => 10,
            4022376103 => 2,
            4024345920 => 12,
            4031249490 => 12,
            4037036970 => 1,
            4037862832 => 10,
            4074379575 => 9,
            4074543187 => 10,
            4086658281 => 9,
            4088093105 => 9,
            4095422895 => 10,
            4095574036 => 6,
            4095615324 => 12,
            4097777520 => 14,
            4105962743 => 12,
            4122056220 => 9,
            4123344466 => 10,
            4124623270 => 1,
            4124788165 => 3,
            4136498852 => 9,
            4142052618 => 10,
            4143007308 => 7,
            4148101412 => 11,
            4156078855 => 9,
            4158566097 => 3,
            4162380809 => 3,
            4165799628 => 6,
            4166981789 => 4,
            4175244083 => 9,
            4182860854 => 0,
            4186316022 => 6,
            4201705270 => 6,
            4207607924 => 9,
            4208778838 => 7,
            4217484030 => 9,
            4218914973 => 14,
            4219587988 => 7,
            4222183408 => 10,
            4231323485 => 10,
            4237592921 => 9,
            4238390223 => 9,
            4240577450 => 4,
            4243806635 => 9,
            4251960020 => 5,
            4252922144 => 13,
            4261334040 => 2,
            4266656042 => 10,
            4278684876 => 8,
            4278956645 => 8,
            4282788508 => 3,
            4288193352 => 9,
            4288270099 => 10,
            4292641817 => 9,
            4294318154 => 0,
            _ => 0,
        },
        IFC_SCHEMA::IFC4X3 => match type_code {
            5716631 => 10,
            15328376 => 2,
            24185140 => 9,
            24726584 => 9,
            25142252 => 9,
            32344328 => 9,
            32440307 => 1,
            33720170 => 9,
            39481116 => 10,
            42703149 => 4,
            45288368 => 5,
            59481748 => 4,
            76236018 => 9,
            90941305 => 9,
            101040310 => 3,
            103090709 => 9,
            110355661 => 8,
            125510826 => 4,
            130549933 => 9,
            132023988 => 10,
            144952367 => 2,
            146592293 => 10,
            148013059 => 11,
            148025276 => 5,
            160246688 => 6,
            164193824 => 4,
            167062518 => 12,
            177149247 => 9,
            178086475 => 3,
            178912537 => 1,
            180925521 => 1,
            182550632 => 7,
            182646315 => 9,
            205026976 => 7,
            211053100 => 7,
            214636428 => 9,
            219451334 => 4,
            220341763 => 1,
            222769930 => 2,
            230924584 => 2,
            231477066 => 10,
            234836483 => 9,
            248100487 => 7,
            263784265 => 8,
            264262732 => 9,
            277319702 => 9,
            279856033 => 6,
            280115917 => 1,
            297599258 => 3,
            300633059 => 10,
            307848117 => 6,
            310824031 => 9,
            315944413 => 3,
            317615605 => 8,
            325726236 => 8,
            331165859 => 9,
            335055490 => 10,
            336235671 => 16,
            338393293 => 10,
            339256511 => 9,
            342316401 => 9,
            346874300 => 10,
            366585022 => 6,
            370225590 => 1,
            374418227 => 5,
            377706215 => 11,
            385403989 => 11,
            388784114 => 3,
            395041908 => 10,
            395920057 => 13,
            400855858 => 10,
            402227799 => 9,
            411424972 => 10,
            413509423 => 9,
            427810014 => 10,
            427948657 => 10,
            428585644 => 12,
            445594917 => 1,
            448429030 => 4,
            451544542 => 2,
            463610769 => 9,
            476780140 => 4,
            477187591 => 4,
            478536968 => 4,
            479945903 => 10,
            484807127 => 9,
            488727124 => 11,
            492091185 => 7,
            504942748 => 11,
            506776471 => 10,
            512836454 => 9,
            514975943 => 10,
            525669439 => 10,
            530289379 => 7,
            531007025 => 2,
            536804194 => 9,
            539742890 => 7,
            544395925 => 4,
            550521510 => 11,
            552965576 => 7,
            562808652 => 7,
            569719735 => 10,
            572779678 => 9,
            574549367 => 0,
            578613899 => 10,
            581633288 => 1,
            590820931 => 1,
            593015953 => 5,
            597895409 => 9,
            602808272 => 10,
            603570806 => 3,
            603775116 => 10,
            609421318 => 1,
            616511568 => 7,
            618182010 => 3,
            618700268 => 10,
            626085974 => 5,
            629592764 => 9,
            630975310 => 9,
            635142910 => 9,
            639361253 => 9,
            639542469 => 4,
            644574406 => 10,
            647756555 => 9,
            647927063 => 6,
            653396225 => 9,
            655969474 => 10,
            663422040 => 10,
            669184980 => 2,
            673634403 => 3,
            677532197 => 0,
            679976338 => 10,
            682877961 => 10,
            683857671 => 13,
            693640335 => 4,
            699246055 => 3,
            707683696 => 8,
            710110818 => 10,
            710998568 => 9,
            712377611 => 10,
            723233188 => 0,
            728799441 => 10,
            734778138 => 9,
            738039164 => 9,
            738692330 => 3,
            747523909 => 7,
            750771296 => 6,
            753842376 => 9,
            759155922 => 1,
            760658860 => 0,
            770865208 => 5,
            775493141 => 2,
            776857604 => 4,
            781010003 => 6,
            782932809 => 5,
            803316827 => 2,
            804291784 => 10,
            807026263 => 1,
            812098782 => 2,
            812556717 => 9,
            816062949 => 4,
            819412036 => 9,
            819618141 => 10,
            823603102 => 1,
            825690147 => 5,
            826625072 => 4,
            840318589 => 9,
            843113511 => 9,
            846575682 => 2,
            852622518 => 3,
            853536259 => 5,
            862014818 => 9,
            867548509 => 5,
            869906466 => 10,
            871118103 => 6,
            886880790 => 6,
            891718957 => 2,
            900683007 => 9,
            901063453 => 0,
            912023232 => 9,
            917726184 => 6,
            919958153 => 6,
            926996030 => 9,
            931644368 => 5,
            941946838 => 4,
            963979645 => 11,
            964333572 => 9,
            976884017 => 11,
            977012517 => 10,
            979691226 => 14,
            982818633 => 6,
            985171141 => 3,
            986844984 => 0,
            987401354 => 8,
            987898635 => 1,
            991950508 => 9,
            1003880860 => 9,
            1004757350 => 12,
            1008929658 => 0,
            1010789467 => 3,
            1027710054 => 8,
            1027922057 => 9,
            1028945134 => 13,
            1029017970 => 4,
            1033248425 => 6,
            1033361043 => 6,
            1039846685 => 10,
            1040185647 => 3,
            1042787934 => 18,
            1045800335 => 2,
            1051575348 => 10,
            1051757585 => 9,
            1052013943 => 9,
            1054537805 => 3,
            1060000209 => 11,
            1062813311 => 8,
            1072016465 => 10,
            1073191201 => 9,
            1076942058 => 4,
            1077100507 => 8,
            1095909175 => 9,
            1096409881 => 6,
            1105321065 => 2,
            1114901282 => 10,
            1123145078 => 1,
            1133259667 => 10,
            1136057603 => 2,
            1154170062 => 17,
            1154579445 => 7,
            1158309216 => 10,
            1161773419 => 10,
            1162798199 => 9,
            1175146630 => 2,
            1177604601 => 7,
            1179482911 => 8,
            1190533807 => 8,
            1199560280 => 2,
            1204542856 => 7,
            1207048766 => 8,
            1209101575 => 9,
            1210645708 => 1,
            1213902940 => 2,
            1217240411 => 10,
            1229763772 => 6,
            1232101972 => 9,
            1235345126 => 9,
            1236880293 => 6,
            1245217292 => 6,
            1251058090 => 10,
            1252848954 => 10,
            1260505505 => 0,
            1260650574 => 5,
            1268542332 => 11,
            1281925730 => 2,
            1285652485 => 10,
            1287392070 => 8,
            1290935644 => 3,
            1300840506 => 3,
            1303795690 => 5,
            1304840413 => 8,
            1305183839 => 10,
            1306400036 => 9,
            1307041759 => 7,
            1310608509 => 3,
            1310830890 => 10,
            1329646415 => 9,
            1334484129 => 4,
            1335981549 => 9,
            1339347760 => 9,
            1351298697 => 1,
            1356537516 => 3,
            1360408905 => 9,
            1377556343 => 0,
            1383045692 => 4,
            1383356374 => 9,
            1387855156 => 7,
            1401173127 => 6,
            1402838566 => 4,
            1404847402 => 9,
            1411181986 => 4,
            1411407467 => 10,
            1412071761 => 8,
            1416205885 => 7,
            1417489154 => 2,
            1419761937 => 10,
            1423911732 => 3,
            1425443689 => 1,
            1426591983 => 9,
            1437502449 => 9,
            1437805879 => 4,
            1437953363 => 3,
            1441486842 => 6,
            1447204868 => 5,
            1451395588 => 5,
            1457835157 => 10,
            1462361463 => 6,
            1466758467 => 3,
            1469900589 => 10,
            1472233963 => 1,
            1482703590 => 4,
            1482959167 => 9,
            1484403080 => 10,
            1485152156 => 4,
            1502416096 => 9,
            1507914824 => 0,
            1509187699 => 3,
            1509553395 => 9,
            1520743889 => 9,
            1521410863 => 11,
            1525564444 => 12,
            1529196076 => 9,
            1530820697 => 9,
            1532957894 => 10,
            1534661035 => 10,
            1545765605 => 7,
            1549132990 => 20,
            1560379544 => 7,
            1566485204 => 2,
            1580146022 => 6,
            1580310250 => 10,
            1585845231 => 5,
            1594536857 => 9,
            1595516126 => 7,
            1597423693 => 7,
            1599208980 => 10,
            1600972822 => 10,
            1607154358 => 2,
            1608871552 => 4,
            1620046519 => 9,
            1621171031 => 12,
            1623761950 => 8,
            1626504194 => 9,
            1628702193 => 6,
            1634111441 => 9,
            1635779807 => 1,
            1638771189 => 10,
            1638804497 => 9,
            1640371178 => 7,
            1658829314 => 8,
            1660063152 => 2,
            1662888072 => 7,
            1663979128 => 2,
            1674181508 => 8,
            1675464909 => 2,
            1677625105 => 8,
            1680319473 => 4,
            1682466193 => 2,
            1683148259 => 8,
            1687234759 => 10,
            1692211062 => 10,
            1704287377 => 3,
            1714330368 => 9,
            1735638870 => 4,
            1742049831 => 3,
            1758889154 => 8,
            1763565496 => 10,
            1765591967 => 4,
            1768891740 => 10,
            1770583370 => 10,
            1775413392 => 1,
            1783015770 => 10,
            1785450214 => 2,
            1794013214 => 5,
            1806887404 => 10,
            1807405624 => 12,
            1809719519 => 2,
            1810631287 => 10,
            1815067380 => 12,
            1834744321 => 9,
            1838606355 => 3,
            1842657554 => 10,
            1847130766 => 2,
            1847252529 => 9,
            1856042241 => 4,
            1862484736 => 2,
            1865459582 => 5,
            1871374353 => 10,
            1876633798 => 8,
            1878645084 => 9,
            1883228015 => 6,
            1891881377 => 11,
            1893162501 => 10,
            1894708472 => 10,
            1898987631 => 10,
            1904799276 => 9,
            1907098498 => 1,
            1909888760 => 10,
            1911125066 => 10,
            1916426348 => 10,
            1918398963 => 2,
            1935646853 => 3,
            1945004755 => 8,
            1946335990 => 7,
            1950438474 => 10,
            1953115116 => 8,
            1959218052 => 7,
            1967976161 => 5,
            1971632696 => 8,
            1973038258 => 8,
            1973544240 => 9,
            1975003073 => 8,
            1981873012 => 2,
            1983826977 => 6,
            1999602285 => 9,
            2000195564 => 3,
            2004835150 => 1,
            2016517767 => 9,
            2022407955 => 4,
            2022622350 => 4,
            2028607225 => 6,
            2030761528 => 10,
            2042790032 => 3,
            2043862942 => 5,
            2044713172 => 5,
            2047409740 => 1,
            2056796094 => 9,
            2058353004 => 8,
            2059837836 => 2,
            2063403501 => 9,
            2067069095 => 0,
            2068733104 => 9,
            2069777674 => 8,
            2077209135 => 8,
            2078563270 => 9,
            2082059205 => 10,
            2090586900 => 4,
            2093928680 => 5,
            2095639259 => 3,
            2097647324 => 10,
            2107101300 => 9,
            2108223431 => 12,
            2127690289 => 6,
            2133299955 => 4,
            2142170206 => 10,
            2143335405 => 8,
            2147822146 => 1,
            2157484638 => 3,
            2162789131 => 1,
            2165702409 => 5,
            2176052936 => 9,
            2176059722 => 7,
            2182337498 => 9,
            2185764099 => 12,
            2188021234 => 9,
            2188180465 => 10,
            2197970202 => 10,
            2199411900 => 1,
            2205249479 => 1,
            2218152070 => 9,
            2223149337 => 8,
            2226359599 => 3,
            2233826070 => 3,
            2235152071 => 6,
            2242383968 => 3,
            2247615214 => 2,
            2250791053 => 10,
            2251480897 => 11,
            2254336722 => 5,
            2262370178 => 9,
            2272882330 => 9,
            2273995522 => 1,
            2281632017 => 10,
            2294589976 => 2,
            2295281155 => 9,
            2296667514 => 6,
            2297155007 => 9,
            2301859152 => 10,
            2310774935 => 20,
            2315554128 => 10,
            2320036040 => 18,
            2323601079 => 13,
            2324767716 => 10,
            2341007311 => 4,
            2347385850 => 2,
            2347447852 => 10,
            2347495698 => 8,
            2367409068 => 3,
            2382730787 => 8,
            2387106220 => 1,
            2391368822 => 11,
            2391383451 => 9,
            2391406946 => 9,
            2397081782 => 10,
            2405470396 => 5,
            2415094496 => 13,
            2417008758 => 10,
            2433181523 => 5,
            2439245199 => 2,
            2445595289 => 9,
            2453401579 => 0,
            2461110595 => 8,
            2473145415 => 7,
            2474470126 => 9,
            2481509218 => 11,
            2483315170 => 2,
            2485617015 => 3,
            2485787929 => 3,
            2489546625 => 10,
            2495723537 => 7,
            2506170314 => 1,
            2510884976 => 1,
            2513912981 => 0,
            2515109513 => 10,
            2519244187 => 1,
            2525727697 => 1,
            2529465313 => 3,
            2533589738 => 10,
            2542286263 => 4,
            2543172580 => 9,
            2551354335 => 4,
            2552916305 => 3,
            2556980723 => 1,
            2559016684 => 1,
            2559216714 => 10,
            2565941209 => 6,
            2568555532 => 9,
            2571569899 => 3,
            2572171363 => 16,
            2574617495 => 11,
            2581212453 => 4,
            2590856083 => 9,
            2597039031 => 2,
            2598011224 => 2,
            2601014836 => 0,
            2603310189 => 2,
            2604431987 => 5,
            2609359061 => 4,
            2611217952 => 2,
            2614616156 => 2,
            2624227202 => 2,
            2629017746 => 3,
            2635815018 => 10,
            2636378356 => 2,
            2652556860 => 6,
            2655187982 => 6,
            2655215786 => 6,
            2665983363 => 1,
            2668620305 => 4,
            2674252688 => 10,
            2680139844 => 8,
            2691318326 => 5,
            2696325953 => 9,
            2705031697 => 4,
            2706460486 => 5,
            2706606064 => 9,
            2706619895 => 1,
            2713105998 => 3,
            2713554722 => 5,
            2713699986 => 8,
            2715220739 => 7,
            2728634034 => 7,
            2732653382 => 2,
            2735484536 => 1,
            2736907675 => 3,
            2740243338 => 3,
            2744685151 => 8,
            2752243245 => 4,
            2757150158 => 10,
            2759199220 => 1,
            2769231204 => 9,
            2770003689 => 8,
            2771591690 => 21,
            2775532180 => 4,
            2777663545 => 1,
            2778083089 => 6,
            2781568857 => 10,
            2798486643 => 4,
            2799835756 => 0,
            2802773753 => 6,
            2802850158 => 4,
            2804161546 => 5,
            2809605785 => 4,
            2814081492 => 9,
            2816379211 => 10,
            2827207264 => 8,
            2827736869 => 3,
            2830218821 => 4,
            2835456948 => 5,
            2837617999 => 10,
            2839578677 => 4,
            2852063980 => 3,
            2853485674 => 8,
            2857406711 => 7,
            2859738748 => 0,
            2874132201 => 10,
            2879124712 => 2,
            2887950389 => 7,
            2889183280 => 4,
            2893384427 => 10,
            2898700619 => 4,
            2898889636 => 8,
            2904328755 => 9,
            2914609552 => 7,
            2916149573 => 5,
            2924175390 => 1,
            2934153892 => 4,
            2937912522 => 5,
            2938176219 => 9,
            2940368186 => 10,
            2943643501 => 4,
            2945172077 => 7,
            2949456006 => 7,
            2951183804 => 10,
            2954562838 => 10,
            2963535650 => 17,
            2979338954 => 9,
            2986769608 => 8,
            2998442950 => 5,
            3001207471 => 10,
            3008276851 => 3,
            3008791417 => 0,
            3009204131 => 11,
            3009222698 => 9,
            3020489413 => 2,
            3021840470 => 6,
            3024970846 => 9,
            3026737570 => 9,
            3027567501 => 9,
            3040386961 => 8,
            3041715199 => 10,
            3049322572 => 4,
            3050246964 => 3,
            3053780830 => 9,
            3057273783 => 8,
            3071239417 => 9,
            3071757647 => 12,
            3079605661 => 3,
            3081323446 => 10,
            3087945054 => 9,
            3101149627 => 8,
            3101698114 => 9,
            3112655638 => 10,
            3113134337 => 3,
            3119450353 => 1,
            3124254112 => 10,
            3124975700 => 5,
            3125803723 => 2,
            3132237377 => 8,
            3136571912 => 7,
            3150382593 => 4,
            3171933400 => 9,
            3174744832 => 10,
            3179687236 => 10,
            3181161470 => 10,
            3190031847 => 7,
            3198132628 => 9,
            3200245327 => 3,
            3203706013 => 10,
            3205830791 => 7,
            3206491090 => 11,
            3207858831 => 15,
            3221913625 => 9,
            3242617779 => 6,
            3243963512 => 5,
            3252649465 => 5,
            3256556792 => 9,
            3264961684 => 1,
            3265635763 => 4,
            3268803585 => 6,
            3277789161 => 10,
            3283111854 => 9,
            3285139300 => 1,
            3290496277 => 9,
            3293443760 => 6,
            3293546465 => 10,
            3295246426 => 11,
            3296154744 => 9,
            3303107099 => 4,
            3303938423 => 3,
            3304561284 => 13,
            3310460725 => 9,
            3313531582 => 10,
            3314249567 => 8,
            3319311131 => 9,
            3327091369 => 9,
            3331915920 => 5,
            3342526732 => 14,
            3352864051 => 10,
            3355820592 => 10,
            3357820518 => 4,
            3367102660 => 4,
            3368373690 => 11,
            3376911765 => 9,
            3377609919 => 2,
            3381221214 => 4,
            3388369263 => 3,
            3390157468 => 10,
            3404854881 => 5,
            3406155212 => 3,
            3408363356 => 4,
            3413951693 => 10,
            3415622556 => 9,
            3419103109 => 9,
            3420628829 => 9,
            3422422726 => 13,
            3425423356 => 3,
            3425753595 => 9,
            3426335179 => 8,
            3448662350 => 6,
            3451746338 => 9,
            3452421091 => 6,
            3454111270 => 7,
            3460190687 => 14,
            3460952963 => 9,
            3465909080 => 4,
            3473067441 => 13,
            3478079324 => 3,
            3486308946 => 5,
            3493046030 => 9,
            3495092785 => 9,
            3497074424 => 2,
            3505215534 => 4,
            3508470533 => 8,
            3510044353 => 2,
            3512223829 => 9,
            3518393246 => 9,
            3521284610 => 4,
            3523091289 => 10,
            3544373492 => 9,
            3548104201 => 3,
            3566463478 => 9,
            3570813810 => 4,
            3571504051 => 9,
            3588315303 => 9,
            3590301190 => 1,
            3593883385 => 5,
            3599934289 => 10,
            3611470254 => 1,
            3612865200 => 9,
            3615266464 => 5,
            3626867408 => 3,
            3630933823 => 3,
            3632507154 => 5,
            3633395639 => 9,
            3640358203 => 9,
            3649129432 => 3,
            3649138523 => 10,
            3649235739 => 4,
            3650150729 => 4,
            3651124850 => 9,
            3651464721 => 10,
            3657597509 => 12,
            3663046924 => 10,
            3663146110 => 12,
            3665877780 => 9,
            3678494232 => 9,
            3689010777 => 9,
            3692461612 => 2,
            3693000487 => 9,
            3694346114 => 9,
            3698973494 => 9,
            3701648758 => 1,
            3708119000 => 5,
            3710013099 => 3,
            3724593414 => 1,
            3727388367 => 1,
            3732053477 => 5,
            3732776249 => 2,
            3736923433 => 9,
            3737207727 => 2,
            3740093272 => 7,
            3741457305 => 9,
            3747195512 => 9,
            3749851601 => 4,
            3752311538 => 9,
            3758799889 => 9,
            3760055223 => 10,
            3765753017 => 6,
            3778827333 => 0,
            3798115385 => 3,
            3798194928 => 9,
            3800577675 => 5,
            3812236995 => 11,
            3815607619 => 10,
            3818125796 => 6,
            3821786052 => 9,
            3824725483 => 17,
            3825984169 => 9,
            3827777499 => 11,
            3840914261 => 6,
            3843373140 => 7,
            3849074793 => 9,
            3850581409 => 10,
            3856911033 => 11,
            3862327254 => 7,
            3869604511 => 4,
            3875453745 => 7,
            3888040117 => 5,
            3893378262 => 9,
            3893394355 => 9,
            3895139033 => 9,
            3898045240 => 11,
            3900360178 => 2,
            3902619387 => 9,
            3905492369 => 6,
            3907093117 => 9,
            3915482550 => 8,
            3939117080 => 6,
            3940055652 => 6,
            3945020480 => 11,
            3946677679 => 10,
            3948183225 => 10,
            3956297820 => 10,
            3958052878 => 3,
            3958567839 => 2,
            3961806047 => 10,
            3967405729 => 4,
            3979015343 => 9,
            3982875396 => 4,
            3992365140 => 10,
            3999819293 => 9,
            4006246654 => 1,
            4009809668 => 13,
            4015995234 => 2,
            4017108033 => 10,
            4021432810 => 8,
            4022376103 => 2,
            4024345920 => 12,
            4031249490 => 12,
            4037036970 => 1,
            4037862832 => 10,
            4074379575 => 9,
            4074543187 => 10,
            4086658281 => 9,
            4088093105 => 9,
            4095422895 => 10,
            4095574036 => 6,
            4095615324 => 12,
            4097777520 => 14,
            4105526436 => 11,
            4105962743 => 12,
            4122056220 => 9,
            4123344466 => 10,
            4124623270 => 1,
            4124788165 => 3,
            4136498852 => 9,
            4142052618 => 10,
            4143007308 => 7,
            4148101412 => 11,
            4158566097 => 3,
            4162380809 => 3,
            4165799628 => 6,
            4166981789 => 4,
            4175244083 => 9,
            4182860854 => 0,
            4186316022 => 6,
            4189326743 => 10,
            4196446775 => 9,
            4201705270 => 6,
            4207607924 => 9,
            4208778838 => 7,
            4212018352 => 5,
            4217484030 => 9,
            4218914973 => 14,
            4219587988 => 7,
            4222183408 => 10,
            4228831410 => 11,
            4230923436 => 8,
            4231323485 => 10,
            4234616927 => 6,
            4237592921 => 9,
            4238390223 => 9,
            4240577450 => 4,
            4243806635 => 9,
            4251960020 => 5,
            4252922144 => 13,
            4261334040 => 2,
            4266260250 => 8,
            4266656042 => 10,
            4278684876 => 8,
            4278956645 => 8,
            4282788508 => 3,
            4288193352 => 9,
            4288270099 => 10,
            4292641817 => 9,
            4294318154 => 0,
            _ => 0,
        },
    }
}
