//! Rust port of IfcSchemaManager.

use std::collections::HashSet;
use std::string::String;
use std::string::ToString;

use super::ifc_schema::{IFC_SCHEMA, IfcTypeCode};

#[derive(Debug)]
pub struct IfcSchemaManager {
    crc_table: Vec<u32>,
    ifc_elements: HashSet<u32>,
    schemas: Vec<IFC_SCHEMA>,
    schema_names: Vec<&'static str>,
}

impl IfcSchemaManager {
    pub fn new() -> Self {
        let mut manager = Self {
            crc_table: vec![0; 256],
            ifc_elements: HashSet::new(),
            schemas: Vec::new(),
            schema_names: Vec::new(),
        };
        manager.init_crc_table();
        manager.init_schema_data();
        manager
    }

    fn init_crc_table(&mut self) {
        for n in 0..256u32 {
            let mut c = n;
            for _ in 0..8 {
                c = if (c & 1) != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            }
            self.crc_table[n as usize] = c;
        }
    }

    fn init_schema_data(&mut self) {
        self.ifc_elements.insert(super::ifc_schema::IFCPRODUCT);
        self.ifc_elements.insert(super::ifc_schema::IFCPROXY);
        self.ifc_elements.insert(super::ifc_schema::IFCSPATIALSTRUCTUREELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALACTIVITY);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALITEM);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALMEMBER);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALREACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALSURFACEMEMBER);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALSURFACEMEMBERVARYING);
        self.ifc_elements.insert(super::ifc_schema::IFCANNOTATION);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILDING);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILDINGSTOREY);
        self.ifc_elements.insert(super::ifc_schema::IFCELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCELEMENTASSEMBLY);
        self.ifc_elements.insert(super::ifc_schema::IFCELEMENTCOMPONENT);
        self.ifc_elements.insert(super::ifc_schema::IFCEQUIPMENTELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCFASTENER);
        self.ifc_elements.insert(super::ifc_schema::IFCFEATUREELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCFEATUREELEMENTADDITION);
        self.ifc_elements.insert(super::ifc_schema::IFCFEATUREELEMENTSUBTRACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCFURNISHINGELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCGRID);
        self.ifc_elements.insert(super::ifc_schema::IFCMECHANICALFASTENER);
        self.ifc_elements.insert(super::ifc_schema::IFCOPENINGELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCPORT);
        self.ifc_elements.insert(super::ifc_schema::IFCPROJECTIONELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCSITE);
        self.ifc_elements.insert(super::ifc_schema::IFCSPACE);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALCONNECTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALCURVECONNECTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALCURVEMEMBER);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALCURVEMEMBERVARYING);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALLINEARACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALLINEARACTIONVARYING);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALPLANARACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALPLANARACTIONVARYING);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALPOINTACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALPOINTCONNECTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALPOINTREACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALSURFACECONNECTION);
        self.ifc_elements.insert(super::ifc_schema::IFCTRANSPORTELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCVIRTUALELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILDINGELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILDINGELEMENTCOMPONENT);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILDINGELEMENTPART);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILDINGELEMENTPROXY);
        self.ifc_elements.insert(super::ifc_schema::IFCCOLUMN);
        self.ifc_elements.insert(super::ifc_schema::IFCCOVERING);
        self.ifc_elements.insert(super::ifc_schema::IFCCURTAINWALL);
        self.ifc_elements.insert(super::ifc_schema::IFCDISCRETEACCESSORY);
        self.ifc_elements.insert(super::ifc_schema::IFCDISTRIBUTIONELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCDISTRIBUTIONFLOWELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCDISTRIBUTIONPORT);
        self.ifc_elements.insert(super::ifc_schema::IFCDOOR);
        self.ifc_elements.insert(super::ifc_schema::IFCEDGEFEATURE);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICALELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCENERGYCONVERSIONDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWCONTROLLER);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWFITTING);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWMOVINGDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWSEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWSTORAGEDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWTERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWTREATMENTDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCFOOTING);
        self.ifc_elements.insert(super::ifc_schema::IFCMEMBER);
        self.ifc_elements.insert(super::ifc_schema::IFCPILE);
        self.ifc_elements.insert(super::ifc_schema::IFCPLATE);
        self.ifc_elements.insert(super::ifc_schema::IFCRAILING);
        self.ifc_elements.insert(super::ifc_schema::IFCRAMP);
        self.ifc_elements.insert(super::ifc_schema::IFCRAMPFLIGHT);
        self.ifc_elements.insert(super::ifc_schema::IFCREINFORCINGELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCREINFORCINGMESH);
        self.ifc_elements.insert(super::ifc_schema::IFCROOF);
        self.ifc_elements.insert(super::ifc_schema::IFCROUNDEDEDGEFEATURE);
        self.ifc_elements.insert(super::ifc_schema::IFCSLAB);
        self.ifc_elements.insert(super::ifc_schema::IFCSTAIR);
        self.ifc_elements.insert(super::ifc_schema::IFCSTAIRFLIGHT);
        self.ifc_elements.insert(super::ifc_schema::IFCTENDON);
        self.ifc_elements.insert(super::ifc_schema::IFCTENDONANCHOR);
        self.ifc_elements.insert(super::ifc_schema::IFCWALL);
        self.ifc_elements.insert(super::ifc_schema::IFCWALLSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCWINDOW);
        self.ifc_elements.insert(super::ifc_schema::IFCBEAM);
        self.ifc_elements.insert(super::ifc_schema::IFCCHAMFEREDGEFEATURE);
        self.ifc_elements.insert(super::ifc_schema::IFCDISTRIBUTIONCHAMBERELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCDISTRIBUTIONCONTROLELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICDISTRIBUTIONPOINT);
        self.ifc_elements.insert(super::ifc_schema::IFCREINFORCINGBAR);
        self.ifc_elements.insert(super::ifc_schema::IFCSPATIALELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCSPATIALZONE);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALSURFACEREACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCEXTERNALSPATIALSTRUCTUREELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCFURNITURE);
        self.ifc_elements.insert(super::ifc_schema::IFCGEOGRAPHICELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCOPENINGSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALCURVEACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALCURVEREACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSTRUCTURALSURFACEACTION);
        self.ifc_elements.insert(super::ifc_schema::IFCSURFACEFEATURE);
        self.ifc_elements.insert(super::ifc_schema::IFCSYSTEMFURNITUREELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCVIBRATIONISOLATOR);
        self.ifc_elements.insert(super::ifc_schema::IFCVOIDINGFEATURE);
        self.ifc_elements.insert(super::ifc_schema::IFCCHIMNEY);
        self.ifc_elements.insert(super::ifc_schema::IFCCIVILELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCCOLUMNSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCDOORSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCENGINE);
        self.ifc_elements.insert(super::ifc_schema::IFCEVAPORATIVECOOLER);
        self.ifc_elements.insert(super::ifc_schema::IFCEVAPORATOR);
        self.ifc_elements.insert(super::ifc_schema::IFCEXTERNALSPATIALELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWMETER);
        self.ifc_elements.insert(super::ifc_schema::IFCHEATEXCHANGER);
        self.ifc_elements.insert(super::ifc_schema::IFCHUMIDIFIER);
        self.ifc_elements.insert(super::ifc_schema::IFCINTERCEPTOR);
        self.ifc_elements.insert(super::ifc_schema::IFCJUNCTIONBOX);
        self.ifc_elements.insert(super::ifc_schema::IFCLAMP);
        self.ifc_elements.insert(super::ifc_schema::IFCLIGHTFIXTURE);
        self.ifc_elements.insert(super::ifc_schema::IFCMEDICALDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCMEMBERSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCMOTORCONNECTION);
        self.ifc_elements.insert(super::ifc_schema::IFCOUTLET);
        self.ifc_elements.insert(super::ifc_schema::IFCPIPEFITTING);
        self.ifc_elements.insert(super::ifc_schema::IFCPIPESEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCPLATESTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCPROTECTIVEDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCPUMP);
        self.ifc_elements.insert(super::ifc_schema::IFCSANITARYTERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCSHADINGDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCSLABELEMENTEDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCSLABSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCSOLARDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCSPACEHEATER);
        self.ifc_elements.insert(super::ifc_schema::IFCSTACKTERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCSWITCHINGDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCTANK);
        self.ifc_elements.insert(super::ifc_schema::IFCTRANSFORMER);
        self.ifc_elements.insert(super::ifc_schema::IFCTUBEBUNDLE);
        self.ifc_elements.insert(super::ifc_schema::IFCUNITARYEQUIPMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCVALVE);
        self.ifc_elements.insert(super::ifc_schema::IFCWALLELEMENTEDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCWASTETERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCWINDOWSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCAIRTERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCAIRTERMINALBOX);
        self.ifc_elements.insert(super::ifc_schema::IFCAIRTOAIRHEATRECOVERY);
        self.ifc_elements.insert(super::ifc_schema::IFCAUDIOVISUALAPPLIANCE);
        self.ifc_elements.insert(super::ifc_schema::IFCBEAMSTANDARDCASE);
        self.ifc_elements.insert(super::ifc_schema::IFCBOILER);
        self.ifc_elements.insert(super::ifc_schema::IFCBURNER);
        self.ifc_elements.insert(super::ifc_schema::IFCCABLECARRIERFITTING);
        self.ifc_elements.insert(super::ifc_schema::IFCCABLECARRIERSEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCCABLEFITTING);
        self.ifc_elements.insert(super::ifc_schema::IFCCABLESEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCCHILLER);
        self.ifc_elements.insert(super::ifc_schema::IFCCOIL);
        self.ifc_elements.insert(super::ifc_schema::IFCCOMMUNICATIONSAPPLIANCE);
        self.ifc_elements.insert(super::ifc_schema::IFCCOMPRESSOR);
        self.ifc_elements.insert(super::ifc_schema::IFCCONDENSER);
        self.ifc_elements.insert(super::ifc_schema::IFCCOOLEDBEAM);
        self.ifc_elements.insert(super::ifc_schema::IFCCOOLINGTOWER);
        self.ifc_elements.insert(super::ifc_schema::IFCDAMPER);
        self.ifc_elements.insert(super::ifc_schema::IFCDUCTFITTING);
        self.ifc_elements.insert(super::ifc_schema::IFCDUCTSEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCDUCTSILENCER);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICAPPLIANCE);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICDISTRIBUTIONBOARD);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICFLOWSTORAGEDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICGENERATOR);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICMOTOR);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICTIMECONTROL);
        self.ifc_elements.insert(super::ifc_schema::IFCFAN);
        self.ifc_elements.insert(super::ifc_schema::IFCFILTER);
        self.ifc_elements.insert(super::ifc_schema::IFCFIRESUPPRESSIONTERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCFLOWINSTRUMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCPROTECTIVEDEVICETRIPPINGUNIT);
        self.ifc_elements.insert(super::ifc_schema::IFCSENSOR);
        self.ifc_elements.insert(super::ifc_schema::IFCUNITARYCONTROLELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCACTUATOR);
        self.ifc_elements.insert(super::ifc_schema::IFCALARM);
        self.ifc_elements.insert(super::ifc_schema::IFCCONTROLLER);
        self.ifc_elements.insert(super::ifc_schema::IFCFACILITY);
        self.ifc_elements.insert(super::ifc_schema::IFCFACILITYPART);
        self.ifc_elements.insert(super::ifc_schema::IFCFACILITYPARTCOMMON);
        self.ifc_elements.insert(super::ifc_schema::IFCGEOTECHNICALELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCGEOTECHNICALSTRATUM);
        self.ifc_elements.insert(super::ifc_schema::IFCIMPACTPROTECTIONDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCLINEARELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCMARINEFACILITY);
        self.ifc_elements.insert(super::ifc_schema::IFCMARINEPART);
        self.ifc_elements.insert(super::ifc_schema::IFCPOSITIONINGELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCRAILWAY);
        self.ifc_elements.insert(super::ifc_schema::IFCRAILWAYPART);
        self.ifc_elements.insert(super::ifc_schema::IFCREFERENT);
        self.ifc_elements.insert(super::ifc_schema::IFCROAD);
        self.ifc_elements.insert(super::ifc_schema::IFCROADPART);
        self.ifc_elements.insert(super::ifc_schema::IFCSIGN);
        self.ifc_elements.insert(super::ifc_schema::IFCTENDONCONDUIT);
        self.ifc_elements.insert(super::ifc_schema::IFCTRANSPORTATIONDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCVEHICLE);
        self.ifc_elements.insert(super::ifc_schema::IFCVIBRATIONDAMPER);
        self.ifc_elements.insert(super::ifc_schema::IFCALIGNMENTCANT);
        self.ifc_elements.insert(super::ifc_schema::IFCALIGNMENTHORIZONTAL);
        self.ifc_elements.insert(super::ifc_schema::IFCALIGNMENTSEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCALIGNMENTVERTICAL);
        self.ifc_elements.insert(super::ifc_schema::IFCBRIDGE);
        self.ifc_elements.insert(super::ifc_schema::IFCBRIDGEPART);
        self.ifc_elements.insert(super::ifc_schema::IFCBUILTELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCCOURSE);
        self.ifc_elements.insert(super::ifc_schema::IFCDEEPFOUNDATION);
        self.ifc_elements.insert(super::ifc_schema::IFCEARTHWORKSCUT);
        self.ifc_elements.insert(super::ifc_schema::IFCEARTHWORKSELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCEARTHWORKSFILL);
        self.ifc_elements.insert(super::ifc_schema::IFCGEOTECHNICALASSEMBLY);
        self.ifc_elements.insert(super::ifc_schema::IFCKERB);
        self.ifc_elements.insert(super::ifc_schema::IFCLINEARPOSITIONINGELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCLIQUIDTERMINAL);
        self.ifc_elements.insert(super::ifc_schema::IFCMOBILETELECOMMUNICATIONSAPPLIANCE);
        self.ifc_elements.insert(super::ifc_schema::IFCMOORINGDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCNAVIGATIONELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCPAVEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCRAIL);
        self.ifc_elements.insert(super::ifc_schema::IFCREINFORCEDSOIL);
        self.ifc_elements.insert(super::ifc_schema::IFCSIGNAL);
        self.ifc_elements.insert(super::ifc_schema::IFCTRACKELEMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCALIGNMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCBEARING);
        self.ifc_elements.insert(super::ifc_schema::IFCBOREHOLE);
        self.ifc_elements.insert(super::ifc_schema::IFCCAISSONFOUNDATION);
        self.ifc_elements.insert(super::ifc_schema::IFCCONVEYORSEGMENT);
        self.ifc_elements.insert(super::ifc_schema::IFCDISTRIBUTIONBOARD);
        self.ifc_elements.insert(super::ifc_schema::IFCELECTRICFLOWTREATMENTDEVICE);
        self.ifc_elements.insert(super::ifc_schema::IFCGEOMODEL);
        self.ifc_elements.insert(super::ifc_schema::IFCGEOSLICE);
        self.schema_names.push("IFC2X3");
        self.schema_names.push("IFC4");
        self.schema_names.push("IFC4X3");
        self.schemas.push(IFC_SCHEMA::IFC2X3);
        self.schemas.push(IFC_SCHEMA::IFC4);
        self.schemas.push(IFC_SCHEMA::IFC4X3);
    }

    pub fn get_available_schemas(&self) -> Vec<IFC_SCHEMA> {
        self.schemas.clone()
    }

    pub fn get_schema_name(&self, schema: IFC_SCHEMA) -> &'static str {
        self.schema_names[schema as usize]
    }

    pub fn ifc_type_to_type_code(&self, name: &str) -> u32 {
        self.ifc_type_to_type_code_bytes(name.as_bytes())
    }

    fn ifc_type_to_type_code_bytes(&self, name: &[u8]) -> u32 {
        let mut c: u32 = 0xFFFF_FFFF;
        for &byte in name {
            let idx = ((c ^ (byte as u32)) & 0xFF) as usize;
            c = self.crc_table[idx] ^ (c >> 8);
        }
        c ^ 0xFFFF_FFFF
    }

    pub fn ifc_type_code_to_type(&self, type_code: u32) -> String {
        match type_code {
            super::ifc_schema::FILE_SCHEMA => "FILE_SCHEMA".to_string(),
            super::ifc_schema::FILE_NAME => "FILE_NAME".to_string(),
            super::ifc_schema::FILE_DESCRIPTION => "FILE_DESCRIPTION".to_string(),
            super::ifc_schema::IFCACTORROLE => "IfcActorRole".to_string(),
            super::ifc_schema::IFCADDRESS => "IfcAddress".to_string(),
            super::ifc_schema::IFCAPPLICATION => "IfcApplication".to_string(),
            super::ifc_schema::IFCAPPLIEDVALUE => "IfcAppliedValue".to_string(),
            super::ifc_schema::IFCAPPLIEDVALUERELATIONSHIP => "IfcAppliedValueRelationship".to_string(),
            super::ifc_schema::IFCAPPROVAL => "IfcApproval".to_string(),
            super::ifc_schema::IFCAPPROVALACTORRELATIONSHIP => "IfcApprovalActorRelationship".to_string(),
            super::ifc_schema::IFCAPPROVALPROPERTYRELATIONSHIP => "IfcApprovalPropertyRelationship".to_string(),
            super::ifc_schema::IFCAPPROVALRELATIONSHIP => "IfcApprovalRelationship".to_string(),
            super::ifc_schema::IFCBOUNDARYCONDITION => "IfcBoundaryCondition".to_string(),
            super::ifc_schema::IFCBOUNDARYEDGECONDITION => "IfcBoundaryEdgeCondition".to_string(),
            super::ifc_schema::IFCBOUNDARYFACECONDITION => "IfcBoundaryFaceCondition".to_string(),
            super::ifc_schema::IFCBOUNDARYNODECONDITION => "IfcBoundaryNodeCondition".to_string(),
            super::ifc_schema::IFCBOUNDARYNODECONDITIONWARPING => "IfcBoundaryNodeConditionWarping".to_string(),
            super::ifc_schema::IFCCALENDARDATE => "IfcCalendarDate".to_string(),
            super::ifc_schema::IFCCLASSIFICATION => "IfcClassification".to_string(),
            super::ifc_schema::IFCCLASSIFICATIONITEM => "IfcClassificationItem".to_string(),
            super::ifc_schema::IFCCLASSIFICATIONITEMRELATIONSHIP => "IfcClassificationItemRelationship".to_string(),
            super::ifc_schema::IFCCLASSIFICATIONNOTATION => "IfcClassificationNotation".to_string(),
            super::ifc_schema::IFCCLASSIFICATIONNOTATIONFACET => "IfcClassificationNotationFacet".to_string(),
            super::ifc_schema::IFCCOLOURSPECIFICATION => "IfcColourSpecification".to_string(),
            super::ifc_schema::IFCCONNECTIONGEOMETRY => "IfcConnectionGeometry".to_string(),
            super::ifc_schema::IFCCONNECTIONPOINTGEOMETRY => "IfcConnectionPointGeometry".to_string(),
            super::ifc_schema::IFCCONNECTIONPORTGEOMETRY => "IfcConnectionPortGeometry".to_string(),
            super::ifc_schema::IFCCONNECTIONSURFACEGEOMETRY => "IfcConnectionSurfaceGeometry".to_string(),
            super::ifc_schema::IFCCONSTRAINT => "IfcConstraint".to_string(),
            super::ifc_schema::IFCCONSTRAINTAGGREGATIONRELATIONSHIP => "IfcConstraintAggregationRelationship".to_string(),
            super::ifc_schema::IFCCONSTRAINTCLASSIFICATIONRELATIONSHIP => "IfcConstraintClassificationRelationship".to_string(),
            super::ifc_schema::IFCCONSTRAINTRELATIONSHIP => "IfcConstraintRelationship".to_string(),
            super::ifc_schema::IFCCOORDINATEDUNIVERSALTIMEOFFSET => "IfcCoordinatedUniversalTimeOffset".to_string(),
            super::ifc_schema::IFCCOSTVALUE => "IfcCostValue".to_string(),
            super::ifc_schema::IFCCURRENCYRELATIONSHIP => "IfcCurrencyRelationship".to_string(),
            super::ifc_schema::IFCCURVESTYLEFONT => "IfcCurveStyleFont".to_string(),
            super::ifc_schema::IFCCURVESTYLEFONTANDSCALING => "IfcCurveStyleFontAndScaling".to_string(),
            super::ifc_schema::IFCCURVESTYLEFONTPATTERN => "IfcCurveStyleFontPattern".to_string(),
            super::ifc_schema::IFCDATEANDTIME => "IfcDateAndTime".to_string(),
            super::ifc_schema::IFCDERIVEDUNIT => "IfcDerivedUnit".to_string(),
            super::ifc_schema::IFCDERIVEDUNITELEMENT => "IfcDerivedUnitElement".to_string(),
            super::ifc_schema::IFCDIMENSIONALEXPONENTS => "IfcDimensionalExponents".to_string(),
            super::ifc_schema::IFCDOCUMENTELECTRONICFORMAT => "IfcDocumentElectronicFormat".to_string(),
            super::ifc_schema::IFCDOCUMENTINFORMATION => "IfcDocumentInformation".to_string(),
            super::ifc_schema::IFCDOCUMENTINFORMATIONRELATIONSHIP => "IfcDocumentInformationRelationship".to_string(),
            super::ifc_schema::IFCDRAUGHTINGCALLOUTRELATIONSHIP => "IfcDraughtingCalloutRelationship".to_string(),
            super::ifc_schema::IFCENVIRONMENTALIMPACTVALUE => "IfcEnvironmentalImpactValue".to_string(),
            super::ifc_schema::IFCEXTERNALREFERENCE => "IfcExternalReference".to_string(),
            super::ifc_schema::IFCEXTERNALLYDEFINEDHATCHSTYLE => "IfcExternallyDefinedHatchStyle".to_string(),
            super::ifc_schema::IFCEXTERNALLYDEFINEDSURFACESTYLE => "IfcExternallyDefinedSurfaceStyle".to_string(),
            super::ifc_schema::IFCEXTERNALLYDEFINEDSYMBOL => "IfcExternallyDefinedSymbol".to_string(),
            super::ifc_schema::IFCEXTERNALLYDEFINEDTEXTFONT => "IfcExternallyDefinedTextFont".to_string(),
            super::ifc_schema::IFCGRIDAXIS => "IfcGridAxis".to_string(),
            super::ifc_schema::IFCIRREGULARTIMESERIESVALUE => "IfcIrregularTimeSeriesValue".to_string(),
            super::ifc_schema::IFCLIBRARYINFORMATION => "IfcLibraryInformation".to_string(),
            super::ifc_schema::IFCLIBRARYREFERENCE => "IfcLibraryReference".to_string(),
            super::ifc_schema::IFCLIGHTDISTRIBUTIONDATA => "IfcLightDistributionData".to_string(),
            super::ifc_schema::IFCLIGHTINTENSITYDISTRIBUTION => "IfcLightIntensityDistribution".to_string(),
            super::ifc_schema::IFCLOCALTIME => "IfcLocalTime".to_string(),
            super::ifc_schema::IFCMATERIAL => "IfcMaterial".to_string(),
            super::ifc_schema::IFCMATERIALCLASSIFICATIONRELATIONSHIP => "IfcMaterialClassificationRelationship".to_string(),
            super::ifc_schema::IFCMATERIALLAYER => "IfcMaterialLayer".to_string(),
            super::ifc_schema::IFCMATERIALLAYERSET => "IfcMaterialLayerSet".to_string(),
            super::ifc_schema::IFCMATERIALLAYERSETUSAGE => "IfcMaterialLayerSetUsage".to_string(),
            super::ifc_schema::IFCMATERIALLIST => "IfcMaterialList".to_string(),
            super::ifc_schema::IFCMATERIALPROPERTIES => "IfcMaterialProperties".to_string(),
            super::ifc_schema::IFCMEASUREWITHUNIT => "IfcMeasureWithUnit".to_string(),
            super::ifc_schema::IFCMECHANICALMATERIALPROPERTIES => "IfcMechanicalMaterialProperties".to_string(),
            super::ifc_schema::IFCMECHANICALSTEELMATERIALPROPERTIES => "IfcMechanicalSteelMaterialProperties".to_string(),
            super::ifc_schema::IFCMETRIC => "IfcMetric".to_string(),
            super::ifc_schema::IFCMONETARYUNIT => "IfcMonetaryUnit".to_string(),
            super::ifc_schema::IFCNAMEDUNIT => "IfcNamedUnit".to_string(),
            super::ifc_schema::IFCOBJECTPLACEMENT => "IfcObjectPlacement".to_string(),
            super::ifc_schema::IFCOBJECTIVE => "IfcObjective".to_string(),
            super::ifc_schema::IFCOPTICALMATERIALPROPERTIES => "IfcOpticalMaterialProperties".to_string(),
            super::ifc_schema::IFCORGANIZATION => "IfcOrganization".to_string(),
            super::ifc_schema::IFCORGANIZATIONRELATIONSHIP => "IfcOrganizationRelationship".to_string(),
            super::ifc_schema::IFCOWNERHISTORY => "IfcOwnerHistory".to_string(),
            super::ifc_schema::IFCPERSON => "IfcPerson".to_string(),
            super::ifc_schema::IFCPERSONANDORGANIZATION => "IfcPersonAndOrganization".to_string(),
            super::ifc_schema::IFCPHYSICALQUANTITY => "IfcPhysicalQuantity".to_string(),
            super::ifc_schema::IFCPHYSICALSIMPLEQUANTITY => "IfcPhysicalSimpleQuantity".to_string(),
            super::ifc_schema::IFCPOSTALADDRESS => "IfcPostalAddress".to_string(),
            super::ifc_schema::IFCPREDEFINEDITEM => "IfcPreDefinedItem".to_string(),
            super::ifc_schema::IFCPREDEFINEDSYMBOL => "IfcPreDefinedSymbol".to_string(),
            super::ifc_schema::IFCPREDEFINEDTERMINATORSYMBOL => "IfcPreDefinedTerminatorSymbol".to_string(),
            super::ifc_schema::IFCPREDEFINEDTEXTFONT => "IfcPreDefinedTextFont".to_string(),
            super::ifc_schema::IFCPRESENTATIONLAYERASSIGNMENT => "IfcPresentationLayerAssignment".to_string(),
            super::ifc_schema::IFCPRESENTATIONLAYERWITHSTYLE => "IfcPresentationLayerWithStyle".to_string(),
            super::ifc_schema::IFCPRESENTATIONSTYLE => "IfcPresentationStyle".to_string(),
            super::ifc_schema::IFCPRESENTATIONSTYLEASSIGNMENT => "IfcPresentationStyleAssignment".to_string(),
            super::ifc_schema::IFCPRODUCTREPRESENTATION => "IfcProductRepresentation".to_string(),
            super::ifc_schema::IFCPRODUCTSOFCOMBUSTIONPROPERTIES => "IfcProductsOfCombustionProperties".to_string(),
            super::ifc_schema::IFCPROFILEDEF => "IfcProfileDef".to_string(),
            super::ifc_schema::IFCPROFILEPROPERTIES => "IfcProfileProperties".to_string(),
            super::ifc_schema::IFCPROPERTY => "IfcProperty".to_string(),
            super::ifc_schema::IFCPROPERTYCONSTRAINTRELATIONSHIP => "IfcPropertyConstraintRelationship".to_string(),
            super::ifc_schema::IFCPROPERTYDEPENDENCYRELATIONSHIP => "IfcPropertyDependencyRelationship".to_string(),
            super::ifc_schema::IFCPROPERTYENUMERATION => "IfcPropertyEnumeration".to_string(),
            super::ifc_schema::IFCQUANTITYAREA => "IfcQuantityArea".to_string(),
            super::ifc_schema::IFCQUANTITYCOUNT => "IfcQuantityCount".to_string(),
            super::ifc_schema::IFCQUANTITYLENGTH => "IfcQuantityLength".to_string(),
            super::ifc_schema::IFCQUANTITYTIME => "IfcQuantityTime".to_string(),
            super::ifc_schema::IFCQUANTITYVOLUME => "IfcQuantityVolume".to_string(),
            super::ifc_schema::IFCQUANTITYWEIGHT => "IfcQuantityWeight".to_string(),
            super::ifc_schema::IFCREFERENCESVALUEDOCUMENT => "IfcReferencesValueDocument".to_string(),
            super::ifc_schema::IFCREINFORCEMENTBARPROPERTIES => "IfcReinforcementBarProperties".to_string(),
            super::ifc_schema::IFCRELAXATION => "IfcRelaxation".to_string(),
            super::ifc_schema::IFCREPRESENTATION => "IfcRepresentation".to_string(),
            super::ifc_schema::IFCREPRESENTATIONCONTEXT => "IfcRepresentationContext".to_string(),
            super::ifc_schema::IFCREPRESENTATIONITEM => "IfcRepresentationItem".to_string(),
            super::ifc_schema::IFCREPRESENTATIONMAP => "IfcRepresentationMap".to_string(),
            super::ifc_schema::IFCRIBPLATEPROFILEPROPERTIES => "IfcRibPlateProfileProperties".to_string(),
            super::ifc_schema::IFCROOT => "IfcRoot".to_string(),
            super::ifc_schema::IFCSIUNIT => "IfcSIUnit".to_string(),
            super::ifc_schema::IFCSECTIONPROPERTIES => "IfcSectionProperties".to_string(),
            super::ifc_schema::IFCSECTIONREINFORCEMENTPROPERTIES => "IfcSectionReinforcementProperties".to_string(),
            super::ifc_schema::IFCSHAPEASPECT => "IfcShapeAspect".to_string(),
            super::ifc_schema::IFCSHAPEMODEL => "IfcShapeModel".to_string(),
            super::ifc_schema::IFCSHAPEREPRESENTATION => "IfcShapeRepresentation".to_string(),
            super::ifc_schema::IFCSIMPLEPROPERTY => "IfcSimpleProperty".to_string(),
            super::ifc_schema::IFCSTRUCTURALCONNECTIONCONDITION => "IfcStructuralConnectionCondition".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOAD => "IfcStructuralLoad".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADSTATIC => "IfcStructuralLoadStatic".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADTEMPERATURE => "IfcStructuralLoadTemperature".to_string(),
            super::ifc_schema::IFCSTYLEMODEL => "IfcStyleModel".to_string(),
            super::ifc_schema::IFCSTYLEDITEM => "IfcStyledItem".to_string(),
            super::ifc_schema::IFCSTYLEDREPRESENTATION => "IfcStyledRepresentation".to_string(),
            super::ifc_schema::IFCSURFACESTYLE => "IfcSurfaceStyle".to_string(),
            super::ifc_schema::IFCSURFACESTYLELIGHTING => "IfcSurfaceStyleLighting".to_string(),
            super::ifc_schema::IFCSURFACESTYLEREFRACTION => "IfcSurfaceStyleRefraction".to_string(),
            super::ifc_schema::IFCSURFACESTYLESHADING => "IfcSurfaceStyleShading".to_string(),
            super::ifc_schema::IFCSURFACESTYLEWITHTEXTURES => "IfcSurfaceStyleWithTextures".to_string(),
            super::ifc_schema::IFCSURFACETEXTURE => "IfcSurfaceTexture".to_string(),
            super::ifc_schema::IFCSYMBOLSTYLE => "IfcSymbolStyle".to_string(),
            super::ifc_schema::IFCTABLE => "IfcTable".to_string(),
            super::ifc_schema::IFCTABLEROW => "IfcTableRow".to_string(),
            super::ifc_schema::IFCTELECOMADDRESS => "IfcTelecomAddress".to_string(),
            super::ifc_schema::IFCTEXTSTYLE => "IfcTextStyle".to_string(),
            super::ifc_schema::IFCTEXTSTYLEFONTMODEL => "IfcTextStyleFontModel".to_string(),
            super::ifc_schema::IFCTEXTSTYLEFORDEFINEDFONT => "IfcTextStyleForDefinedFont".to_string(),
            super::ifc_schema::IFCTEXTSTYLETEXTMODEL => "IfcTextStyleTextModel".to_string(),
            super::ifc_schema::IFCTEXTSTYLEWITHBOXCHARACTERISTICS => "IfcTextStyleWithBoxCharacteristics".to_string(),
            super::ifc_schema::IFCTEXTURECOORDINATE => "IfcTextureCoordinate".to_string(),
            super::ifc_schema::IFCTEXTURECOORDINATEGENERATOR => "IfcTextureCoordinateGenerator".to_string(),
            super::ifc_schema::IFCTEXTUREMAP => "IfcTextureMap".to_string(),
            super::ifc_schema::IFCTEXTUREVERTEX => "IfcTextureVertex".to_string(),
            super::ifc_schema::IFCTHERMALMATERIALPROPERTIES => "IfcThermalMaterialProperties".to_string(),
            super::ifc_schema::IFCTIMESERIES => "IfcTimeSeries".to_string(),
            super::ifc_schema::IFCTIMESERIESREFERENCERELATIONSHIP => "IfcTimeSeriesReferenceRelationship".to_string(),
            super::ifc_schema::IFCTIMESERIESVALUE => "IfcTimeSeriesValue".to_string(),
            super::ifc_schema::IFCTOPOLOGICALREPRESENTATIONITEM => "IfcTopologicalRepresentationItem".to_string(),
            super::ifc_schema::IFCTOPOLOGYREPRESENTATION => "IfcTopologyRepresentation".to_string(),
            super::ifc_schema::IFCUNITASSIGNMENT => "IfcUnitAssignment".to_string(),
            super::ifc_schema::IFCVERTEX => "IfcVertex".to_string(),
            super::ifc_schema::IFCVERTEXBASEDTEXTUREMAP => "IfcVertexBasedTextureMap".to_string(),
            super::ifc_schema::IFCVERTEXPOINT => "IfcVertexPoint".to_string(),
            super::ifc_schema::IFCVIRTUALGRIDINTERSECTION => "IfcVirtualGridIntersection".to_string(),
            super::ifc_schema::IFCWATERPROPERTIES => "IfcWaterProperties".to_string(),
            super::ifc_schema::IFCANNOTATIONOCCURRENCE => "IfcAnnotationOccurrence".to_string(),
            super::ifc_schema::IFCANNOTATIONSURFACEOCCURRENCE => "IfcAnnotationSurfaceOccurrence".to_string(),
            super::ifc_schema::IFCANNOTATIONSYMBOLOCCURRENCE => "IfcAnnotationSymbolOccurrence".to_string(),
            super::ifc_schema::IFCANNOTATIONTEXTOCCURRENCE => "IfcAnnotationTextOccurrence".to_string(),
            super::ifc_schema::IFCARBITRARYCLOSEDPROFILEDEF => "IfcArbitraryClosedProfileDef".to_string(),
            super::ifc_schema::IFCARBITRARYOPENPROFILEDEF => "IfcArbitraryOpenProfileDef".to_string(),
            super::ifc_schema::IFCARBITRARYPROFILEDEFWITHVOIDS => "IfcArbitraryProfileDefWithVoids".to_string(),
            super::ifc_schema::IFCBLOBTEXTURE => "IfcBlobTexture".to_string(),
            super::ifc_schema::IFCCENTERLINEPROFILEDEF => "IfcCenterLineProfileDef".to_string(),
            super::ifc_schema::IFCCLASSIFICATIONREFERENCE => "IfcClassificationReference".to_string(),
            super::ifc_schema::IFCCOLOURRGB => "IfcColourRgb".to_string(),
            super::ifc_schema::IFCCOMPLEXPROPERTY => "IfcComplexProperty".to_string(),
            super::ifc_schema::IFCCOMPOSITEPROFILEDEF => "IfcCompositeProfileDef".to_string(),
            super::ifc_schema::IFCCONNECTEDFACESET => "IfcConnectedFaceSet".to_string(),
            super::ifc_schema::IFCCONNECTIONCURVEGEOMETRY => "IfcConnectionCurveGeometry".to_string(),
            super::ifc_schema::IFCCONNECTIONPOINTECCENTRICITY => "IfcConnectionPointEccentricity".to_string(),
            super::ifc_schema::IFCCONTEXTDEPENDENTUNIT => "IfcContextDependentUnit".to_string(),
            super::ifc_schema::IFCCONVERSIONBASEDUNIT => "IfcConversionBasedUnit".to_string(),
            super::ifc_schema::IFCCURVESTYLE => "IfcCurveStyle".to_string(),
            super::ifc_schema::IFCDERIVEDPROFILEDEF => "IfcDerivedProfileDef".to_string(),
            super::ifc_schema::IFCDIMENSIONCALLOUTRELATIONSHIP => "IfcDimensionCalloutRelationship".to_string(),
            super::ifc_schema::IFCDIMENSIONPAIR => "IfcDimensionPair".to_string(),
            super::ifc_schema::IFCDOCUMENTREFERENCE => "IfcDocumentReference".to_string(),
            super::ifc_schema::IFCDRAUGHTINGPREDEFINEDTEXTFONT => "IfcDraughtingPreDefinedTextFont".to_string(),
            super::ifc_schema::IFCEDGE => "IfcEdge".to_string(),
            super::ifc_schema::IFCEDGECURVE => "IfcEdgeCurve".to_string(),
            super::ifc_schema::IFCEXTENDEDMATERIALPROPERTIES => "IfcExtendedMaterialProperties".to_string(),
            super::ifc_schema::IFCFACE => "IfcFace".to_string(),
            super::ifc_schema::IFCFACEBOUND => "IfcFaceBound".to_string(),
            super::ifc_schema::IFCFACEOUTERBOUND => "IfcFaceOuterBound".to_string(),
            super::ifc_schema::IFCFACESURFACE => "IfcFaceSurface".to_string(),
            super::ifc_schema::IFCFAILURECONNECTIONCONDITION => "IfcFailureConnectionCondition".to_string(),
            super::ifc_schema::IFCFILLAREASTYLE => "IfcFillAreaStyle".to_string(),
            super::ifc_schema::IFCFUELPROPERTIES => "IfcFuelProperties".to_string(),
            super::ifc_schema::IFCGENERALMATERIALPROPERTIES => "IfcGeneralMaterialProperties".to_string(),
            super::ifc_schema::IFCGENERALPROFILEPROPERTIES => "IfcGeneralProfileProperties".to_string(),
            super::ifc_schema::IFCGEOMETRICREPRESENTATIONCONTEXT => "IfcGeometricRepresentationContext".to_string(),
            super::ifc_schema::IFCGEOMETRICREPRESENTATIONITEM => "IfcGeometricRepresentationItem".to_string(),
            super::ifc_schema::IFCGEOMETRICREPRESENTATIONSUBCONTEXT => "IfcGeometricRepresentationSubContext".to_string(),
            super::ifc_schema::IFCGEOMETRICSET => "IfcGeometricSet".to_string(),
            super::ifc_schema::IFCGRIDPLACEMENT => "IfcGridPlacement".to_string(),
            super::ifc_schema::IFCHALFSPACESOLID => "IfcHalfSpaceSolid".to_string(),
            super::ifc_schema::IFCHYGROSCOPICMATERIALPROPERTIES => "IfcHygroscopicMaterialProperties".to_string(),
            super::ifc_schema::IFCIMAGETEXTURE => "IfcImageTexture".to_string(),
            super::ifc_schema::IFCIRREGULARTIMESERIES => "IfcIrregularTimeSeries".to_string(),
            super::ifc_schema::IFCLIGHTSOURCE => "IfcLightSource".to_string(),
            super::ifc_schema::IFCLIGHTSOURCEAMBIENT => "IfcLightSourceAmbient".to_string(),
            super::ifc_schema::IFCLIGHTSOURCEDIRECTIONAL => "IfcLightSourceDirectional".to_string(),
            super::ifc_schema::IFCLIGHTSOURCEGONIOMETRIC => "IfcLightSourceGoniometric".to_string(),
            super::ifc_schema::IFCLIGHTSOURCEPOSITIONAL => "IfcLightSourcePositional".to_string(),
            super::ifc_schema::IFCLIGHTSOURCESPOT => "IfcLightSourceSpot".to_string(),
            super::ifc_schema::IFCLOCALPLACEMENT => "IfcLocalPlacement".to_string(),
            super::ifc_schema::IFCLOOP => "IfcLoop".to_string(),
            super::ifc_schema::IFCMAPPEDITEM => "IfcMappedItem".to_string(),
            super::ifc_schema::IFCMATERIALDEFINITIONREPRESENTATION => "IfcMaterialDefinitionRepresentation".to_string(),
            super::ifc_schema::IFCMECHANICALCONCRETEMATERIALPROPERTIES => "IfcMechanicalConcreteMaterialProperties".to_string(),
            super::ifc_schema::IFCOBJECTDEFINITION => "IfcObjectDefinition".to_string(),
            super::ifc_schema::IFCONEDIRECTIONREPEATFACTOR => "IfcOneDirectionRepeatFactor".to_string(),
            super::ifc_schema::IFCOPENSHELL => "IfcOpenShell".to_string(),
            super::ifc_schema::IFCORIENTEDEDGE => "IfcOrientedEdge".to_string(),
            super::ifc_schema::IFCPARAMETERIZEDPROFILEDEF => "IfcParameterizedProfileDef".to_string(),
            super::ifc_schema::IFCPATH => "IfcPath".to_string(),
            super::ifc_schema::IFCPHYSICALCOMPLEXQUANTITY => "IfcPhysicalComplexQuantity".to_string(),
            super::ifc_schema::IFCPIXELTEXTURE => "IfcPixelTexture".to_string(),
            super::ifc_schema::IFCPLACEMENT => "IfcPlacement".to_string(),
            super::ifc_schema::IFCPLANAREXTENT => "IfcPlanarExtent".to_string(),
            super::ifc_schema::IFCPOINT => "IfcPoint".to_string(),
            super::ifc_schema::IFCPOINTONCURVE => "IfcPointOnCurve".to_string(),
            super::ifc_schema::IFCPOINTONSURFACE => "IfcPointOnSurface".to_string(),
            super::ifc_schema::IFCPOLYLOOP => "IfcPolyLoop".to_string(),
            super::ifc_schema::IFCPOLYGONALBOUNDEDHALFSPACE => "IfcPolygonalBoundedHalfSpace".to_string(),
            super::ifc_schema::IFCPREDEFINEDCOLOUR => "IfcPreDefinedColour".to_string(),
            super::ifc_schema::IFCPREDEFINEDCURVEFONT => "IfcPreDefinedCurveFont".to_string(),
            super::ifc_schema::IFCPREDEFINEDDIMENSIONSYMBOL => "IfcPreDefinedDimensionSymbol".to_string(),
            super::ifc_schema::IFCPREDEFINEDPOINTMARKERSYMBOL => "IfcPreDefinedPointMarkerSymbol".to_string(),
            super::ifc_schema::IFCPRODUCTDEFINITIONSHAPE => "IfcProductDefinitionShape".to_string(),
            super::ifc_schema::IFCPROPERTYBOUNDEDVALUE => "IfcPropertyBoundedValue".to_string(),
            super::ifc_schema::IFCPROPERTYDEFINITION => "IfcPropertyDefinition".to_string(),
            super::ifc_schema::IFCPROPERTYENUMERATEDVALUE => "IfcPropertyEnumeratedValue".to_string(),
            super::ifc_schema::IFCPROPERTYLISTVALUE => "IfcPropertyListValue".to_string(),
            super::ifc_schema::IFCPROPERTYREFERENCEVALUE => "IfcPropertyReferenceValue".to_string(),
            super::ifc_schema::IFCPROPERTYSETDEFINITION => "IfcPropertySetDefinition".to_string(),
            super::ifc_schema::IFCPROPERTYSINGLEVALUE => "IfcPropertySingleValue".to_string(),
            super::ifc_schema::IFCPROPERTYTABLEVALUE => "IfcPropertyTableValue".to_string(),
            super::ifc_schema::IFCRECTANGLEPROFILEDEF => "IfcRectangleProfileDef".to_string(),
            super::ifc_schema::IFCREGULARTIMESERIES => "IfcRegularTimeSeries".to_string(),
            super::ifc_schema::IFCREINFORCEMENTDEFINITIONPROPERTIES => "IfcReinforcementDefinitionProperties".to_string(),
            super::ifc_schema::IFCRELATIONSHIP => "IfcRelationship".to_string(),
            super::ifc_schema::IFCROUNDEDRECTANGLEPROFILEDEF => "IfcRoundedRectangleProfileDef".to_string(),
            super::ifc_schema::IFCSECTIONEDSPINE => "IfcSectionedSpine".to_string(),
            super::ifc_schema::IFCSERVICELIFEFACTOR => "IfcServiceLifeFactor".to_string(),
            super::ifc_schema::IFCSHELLBASEDSURFACEMODEL => "IfcShellBasedSurfaceModel".to_string(),
            super::ifc_schema::IFCSLIPPAGECONNECTIONCONDITION => "IfcSlippageConnectionCondition".to_string(),
            super::ifc_schema::IFCSOLIDMODEL => "IfcSolidModel".to_string(),
            super::ifc_schema::IFCSOUNDPROPERTIES => "IfcSoundProperties".to_string(),
            super::ifc_schema::IFCSOUNDVALUE => "IfcSoundValue".to_string(),
            super::ifc_schema::IFCSPACETHERMALLOADPROPERTIES => "IfcSpaceThermalLoadProperties".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADLINEARFORCE => "IfcStructuralLoadLinearForce".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADPLANARFORCE => "IfcStructuralLoadPlanarForce".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADSINGLEDISPLACEMENT => "IfcStructuralLoadSingleDisplacement".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADSINGLEDISPLACEMENTDISTORTION => "IfcStructuralLoadSingleDisplacementDistortion".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADSINGLEFORCE => "IfcStructuralLoadSingleForce".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADSINGLEFORCEWARPING => "IfcStructuralLoadSingleForceWarping".to_string(),
            super::ifc_schema::IFCSTRUCTURALPROFILEPROPERTIES => "IfcStructuralProfileProperties".to_string(),
            super::ifc_schema::IFCSTRUCTURALSTEELPROFILEPROPERTIES => "IfcStructuralSteelProfileProperties".to_string(),
            super::ifc_schema::IFCSUBEDGE => "IfcSubedge".to_string(),
            super::ifc_schema::IFCSURFACE => "IfcSurface".to_string(),
            super::ifc_schema::IFCSURFACESTYLERENDERING => "IfcSurfaceStyleRendering".to_string(),
            super::ifc_schema::IFCSWEPTAREASOLID => "IfcSweptAreaSolid".to_string(),
            super::ifc_schema::IFCSWEPTDISKSOLID => "IfcSweptDiskSolid".to_string(),
            super::ifc_schema::IFCSWEPTSURFACE => "IfcSweptSurface".to_string(),
            super::ifc_schema::IFCTSHAPEPROFILEDEF => "IfcTShapeProfileDef".to_string(),
            super::ifc_schema::IFCTERMINATORSYMBOL => "IfcTerminatorSymbol".to_string(),
            super::ifc_schema::IFCTEXTLITERAL => "IfcTextLiteral".to_string(),
            super::ifc_schema::IFCTEXTLITERALWITHEXTENT => "IfcTextLiteralWithExtent".to_string(),
            super::ifc_schema::IFCTRAPEZIUMPROFILEDEF => "IfcTrapeziumProfileDef".to_string(),
            super::ifc_schema::IFCTWODIRECTIONREPEATFACTOR => "IfcTwoDirectionRepeatFactor".to_string(),
            super::ifc_schema::IFCTYPEOBJECT => "IfcTypeObject".to_string(),
            super::ifc_schema::IFCTYPEPRODUCT => "IfcTypeProduct".to_string(),
            super::ifc_schema::IFCUSHAPEPROFILEDEF => "IfcUShapeProfileDef".to_string(),
            super::ifc_schema::IFCVECTOR => "IfcVector".to_string(),
            super::ifc_schema::IFCVERTEXLOOP => "IfcVertexLoop".to_string(),
            super::ifc_schema::IFCWINDOWLININGPROPERTIES => "IfcWindowLiningProperties".to_string(),
            super::ifc_schema::IFCWINDOWPANELPROPERTIES => "IfcWindowPanelProperties".to_string(),
            super::ifc_schema::IFCWINDOWSTYLE => "IfcWindowStyle".to_string(),
            super::ifc_schema::IFCZSHAPEPROFILEDEF => "IfcZShapeProfileDef".to_string(),
            super::ifc_schema::IFCANNOTATIONCURVEOCCURRENCE => "IfcAnnotationCurveOccurrence".to_string(),
            super::ifc_schema::IFCANNOTATIONFILLAREA => "IfcAnnotationFillArea".to_string(),
            super::ifc_schema::IFCANNOTATIONFILLAREAOCCURRENCE => "IfcAnnotationFillAreaOccurrence".to_string(),
            super::ifc_schema::IFCANNOTATIONSURFACE => "IfcAnnotationSurface".to_string(),
            super::ifc_schema::IFCAXIS1PLACEMENT => "IfcAxis1Placement".to_string(),
            super::ifc_schema::IFCAXIS2PLACEMENT2D => "IfcAxis2Placement2D".to_string(),
            super::ifc_schema::IFCAXIS2PLACEMENT3D => "IfcAxis2Placement3D".to_string(),
            super::ifc_schema::IFCBOOLEANRESULT => "IfcBooleanResult".to_string(),
            super::ifc_schema::IFCBOUNDEDSURFACE => "IfcBoundedSurface".to_string(),
            super::ifc_schema::IFCBOUNDINGBOX => "IfcBoundingBox".to_string(),
            super::ifc_schema::IFCBOXEDHALFSPACE => "IfcBoxedHalfSpace".to_string(),
            super::ifc_schema::IFCCSHAPEPROFILEDEF => "IfcCShapeProfileDef".to_string(),
            super::ifc_schema::IFCCARTESIANPOINT => "IfcCartesianPoint".to_string(),
            super::ifc_schema::IFCCARTESIANTRANSFORMATIONOPERATOR => "IfcCartesianTransformationOperator".to_string(),
            super::ifc_schema::IFCCARTESIANTRANSFORMATIONOPERATOR2D => "IfcCartesianTransformationOperator2D".to_string(),
            super::ifc_schema::IFCCARTESIANTRANSFORMATIONOPERATOR2DNONUNIFORM => "IfcCartesianTransformationOperator2DnonUniform".to_string(),
            super::ifc_schema::IFCCARTESIANTRANSFORMATIONOPERATOR3D => "IfcCartesianTransformationOperator3D".to_string(),
            super::ifc_schema::IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM => "IfcCartesianTransformationOperator3DnonUniform".to_string(),
            super::ifc_schema::IFCCIRCLEPROFILEDEF => "IfcCircleProfileDef".to_string(),
            super::ifc_schema::IFCCLOSEDSHELL => "IfcClosedShell".to_string(),
            super::ifc_schema::IFCCOMPOSITECURVESEGMENT => "IfcCompositeCurveSegment".to_string(),
            super::ifc_schema::IFCCRANERAILASHAPEPROFILEDEF => "IfcCraneRailAShapeProfileDef".to_string(),
            super::ifc_schema::IFCCRANERAILFSHAPEPROFILEDEF => "IfcCraneRailFShapeProfileDef".to_string(),
            super::ifc_schema::IFCCSGPRIMITIVE3D => "IfcCsgPrimitive3D".to_string(),
            super::ifc_schema::IFCCSGSOLID => "IfcCsgSolid".to_string(),
            super::ifc_schema::IFCCURVE => "IfcCurve".to_string(),
            super::ifc_schema::IFCCURVEBOUNDEDPLANE => "IfcCurveBoundedPlane".to_string(),
            super::ifc_schema::IFCDEFINEDSYMBOL => "IfcDefinedSymbol".to_string(),
            super::ifc_schema::IFCDIMENSIONCURVE => "IfcDimensionCurve".to_string(),
            super::ifc_schema::IFCDIMENSIONCURVETERMINATOR => "IfcDimensionCurveTerminator".to_string(),
            super::ifc_schema::IFCDIRECTION => "IfcDirection".to_string(),
            super::ifc_schema::IFCDOORLININGPROPERTIES => "IfcDoorLiningProperties".to_string(),
            super::ifc_schema::IFCDOORPANELPROPERTIES => "IfcDoorPanelProperties".to_string(),
            super::ifc_schema::IFCDOORSTYLE => "IfcDoorStyle".to_string(),
            super::ifc_schema::IFCDRAUGHTINGCALLOUT => "IfcDraughtingCallout".to_string(),
            super::ifc_schema::IFCDRAUGHTINGPREDEFINEDCOLOUR => "IfcDraughtingPreDefinedColour".to_string(),
            super::ifc_schema::IFCDRAUGHTINGPREDEFINEDCURVEFONT => "IfcDraughtingPreDefinedCurveFont".to_string(),
            super::ifc_schema::IFCEDGELOOP => "IfcEdgeLoop".to_string(),
            super::ifc_schema::IFCELEMENTQUANTITY => "IfcElementQuantity".to_string(),
            super::ifc_schema::IFCELEMENTTYPE => "IfcElementType".to_string(),
            super::ifc_schema::IFCELEMENTARYSURFACE => "IfcElementarySurface".to_string(),
            super::ifc_schema::IFCELLIPSEPROFILEDEF => "IfcEllipseProfileDef".to_string(),
            super::ifc_schema::IFCENERGYPROPERTIES => "IfcEnergyProperties".to_string(),
            super::ifc_schema::IFCEXTRUDEDAREASOLID => "IfcExtrudedAreaSolid".to_string(),
            super::ifc_schema::IFCFACEBASEDSURFACEMODEL => "IfcFaceBasedSurfaceModel".to_string(),
            super::ifc_schema::IFCFILLAREASTYLEHATCHING => "IfcFillAreaStyleHatching".to_string(),
            super::ifc_schema::IFCFILLAREASTYLETILESYMBOLWITHSTYLE => "IfcFillAreaStyleTileSymbolWithStyle".to_string(),
            super::ifc_schema::IFCFILLAREASTYLETILES => "IfcFillAreaStyleTiles".to_string(),
            super::ifc_schema::IFCFLUIDFLOWPROPERTIES => "IfcFluidFlowProperties".to_string(),
            super::ifc_schema::IFCFURNISHINGELEMENTTYPE => "IfcFurnishingElementType".to_string(),
            super::ifc_schema::IFCFURNITURETYPE => "IfcFurnitureType".to_string(),
            super::ifc_schema::IFCGEOMETRICCURVESET => "IfcGeometricCurveSet".to_string(),
            super::ifc_schema::IFCISHAPEPROFILEDEF => "IfcIShapeProfileDef".to_string(),
            super::ifc_schema::IFCLSHAPEPROFILEDEF => "IfcLShapeProfileDef".to_string(),
            super::ifc_schema::IFCLINE => "IfcLine".to_string(),
            super::ifc_schema::IFCMANIFOLDSOLIDBREP => "IfcManifoldSolidBrep".to_string(),
            super::ifc_schema::IFCOBJECT => "IfcObject".to_string(),
            super::ifc_schema::IFCOFFSETCURVE2D => "IfcOffsetCurve2D".to_string(),
            super::ifc_schema::IFCOFFSETCURVE3D => "IfcOffsetCurve3D".to_string(),
            super::ifc_schema::IFCPERMEABLECOVERINGPROPERTIES => "IfcPermeableCoveringProperties".to_string(),
            super::ifc_schema::IFCPLANARBOX => "IfcPlanarBox".to_string(),
            super::ifc_schema::IFCPLANE => "IfcPlane".to_string(),
            super::ifc_schema::IFCPROCESS => "IfcProcess".to_string(),
            super::ifc_schema::IFCPRODUCT => "IfcProduct".to_string(),
            super::ifc_schema::IFCPROJECT => "IfcProject".to_string(),
            super::ifc_schema::IFCPROJECTIONCURVE => "IfcProjectionCurve".to_string(),
            super::ifc_schema::IFCPROPERTYSET => "IfcPropertySet".to_string(),
            super::ifc_schema::IFCPROXY => "IfcProxy".to_string(),
            super::ifc_schema::IFCRECTANGLEHOLLOWPROFILEDEF => "IfcRectangleHollowProfileDef".to_string(),
            super::ifc_schema::IFCRECTANGULARPYRAMID => "IfcRectangularPyramid".to_string(),
            super::ifc_schema::IFCRECTANGULARTRIMMEDSURFACE => "IfcRectangularTrimmedSurface".to_string(),
            super::ifc_schema::IFCRELASSIGNS => "IfcRelAssigns".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOACTOR => "IfcRelAssignsToActor".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOCONTROL => "IfcRelAssignsToControl".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOGROUP => "IfcRelAssignsToGroup".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOPROCESS => "IfcRelAssignsToProcess".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOPRODUCT => "IfcRelAssignsToProduct".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOPROJECTORDER => "IfcRelAssignsToProjectOrder".to_string(),
            super::ifc_schema::IFCRELASSIGNSTORESOURCE => "IfcRelAssignsToResource".to_string(),
            super::ifc_schema::IFCRELASSOCIATES => "IfcRelAssociates".to_string(),
            super::ifc_schema::IFCRELASSOCIATESAPPLIEDVALUE => "IfcRelAssociatesAppliedValue".to_string(),
            super::ifc_schema::IFCRELASSOCIATESAPPROVAL => "IfcRelAssociatesApproval".to_string(),
            super::ifc_schema::IFCRELASSOCIATESCLASSIFICATION => "IfcRelAssociatesClassification".to_string(),
            super::ifc_schema::IFCRELASSOCIATESCONSTRAINT => "IfcRelAssociatesConstraint".to_string(),
            super::ifc_schema::IFCRELASSOCIATESDOCUMENT => "IfcRelAssociatesDocument".to_string(),
            super::ifc_schema::IFCRELASSOCIATESLIBRARY => "IfcRelAssociatesLibrary".to_string(),
            super::ifc_schema::IFCRELASSOCIATESMATERIAL => "IfcRelAssociatesMaterial".to_string(),
            super::ifc_schema::IFCRELASSOCIATESPROFILEPROPERTIES => "IfcRelAssociatesProfileProperties".to_string(),
            super::ifc_schema::IFCRELCONNECTS => "IfcRelConnects".to_string(),
            super::ifc_schema::IFCRELCONNECTSELEMENTS => "IfcRelConnectsElements".to_string(),
            super::ifc_schema::IFCRELCONNECTSPATHELEMENTS => "IfcRelConnectsPathElements".to_string(),
            super::ifc_schema::IFCRELCONNECTSPORTTOELEMENT => "IfcRelConnectsPortToElement".to_string(),
            super::ifc_schema::IFCRELCONNECTSPORTS => "IfcRelConnectsPorts".to_string(),
            super::ifc_schema::IFCRELCONNECTSSTRUCTURALACTIVITY => "IfcRelConnectsStructuralActivity".to_string(),
            super::ifc_schema::IFCRELCONNECTSSTRUCTURALELEMENT => "IfcRelConnectsStructuralElement".to_string(),
            super::ifc_schema::IFCRELCONNECTSSTRUCTURALMEMBER => "IfcRelConnectsStructuralMember".to_string(),
            super::ifc_schema::IFCRELCONNECTSWITHECCENTRICITY => "IfcRelConnectsWithEccentricity".to_string(),
            super::ifc_schema::IFCRELCONNECTSWITHREALIZINGELEMENTS => "IfcRelConnectsWithRealizingElements".to_string(),
            super::ifc_schema::IFCRELCONTAINEDINSPATIALSTRUCTURE => "IfcRelContainedInSpatialStructure".to_string(),
            super::ifc_schema::IFCRELCOVERSBLDGELEMENTS => "IfcRelCoversBldgElements".to_string(),
            super::ifc_schema::IFCRELCOVERSSPACES => "IfcRelCoversSpaces".to_string(),
            super::ifc_schema::IFCRELDECOMPOSES => "IfcRelDecomposes".to_string(),
            super::ifc_schema::IFCRELDEFINES => "IfcRelDefines".to_string(),
            super::ifc_schema::IFCRELDEFINESBYPROPERTIES => "IfcRelDefinesByProperties".to_string(),
            super::ifc_schema::IFCRELDEFINESBYTYPE => "IfcRelDefinesByType".to_string(),
            super::ifc_schema::IFCRELFILLSELEMENT => "IfcRelFillsElement".to_string(),
            super::ifc_schema::IFCRELFLOWCONTROLELEMENTS => "IfcRelFlowControlElements".to_string(),
            super::ifc_schema::IFCRELINTERACTIONREQUIREMENTS => "IfcRelInteractionRequirements".to_string(),
            super::ifc_schema::IFCRELNESTS => "IfcRelNests".to_string(),
            super::ifc_schema::IFCRELOCCUPIESSPACES => "IfcRelOccupiesSpaces".to_string(),
            super::ifc_schema::IFCRELOVERRIDESPROPERTIES => "IfcRelOverridesProperties".to_string(),
            super::ifc_schema::IFCRELPROJECTSELEMENT => "IfcRelProjectsElement".to_string(),
            super::ifc_schema::IFCRELREFERENCEDINSPATIALSTRUCTURE => "IfcRelReferencedInSpatialStructure".to_string(),
            super::ifc_schema::IFCRELSCHEDULESCOSTITEMS => "IfcRelSchedulesCostItems".to_string(),
            super::ifc_schema::IFCRELSEQUENCE => "IfcRelSequence".to_string(),
            super::ifc_schema::IFCRELSERVICESBUILDINGS => "IfcRelServicesBuildings".to_string(),
            super::ifc_schema::IFCRELSPACEBOUNDARY => "IfcRelSpaceBoundary".to_string(),
            super::ifc_schema::IFCRELVOIDSELEMENT => "IfcRelVoidsElement".to_string(),
            super::ifc_schema::IFCRESOURCE => "IfcResource".to_string(),
            super::ifc_schema::IFCREVOLVEDAREASOLID => "IfcRevolvedAreaSolid".to_string(),
            super::ifc_schema::IFCRIGHTCIRCULARCONE => "IfcRightCircularCone".to_string(),
            super::ifc_schema::IFCRIGHTCIRCULARCYLINDER => "IfcRightCircularCylinder".to_string(),
            super::ifc_schema::IFCSPATIALSTRUCTUREELEMENT => "IfcSpatialStructureElement".to_string(),
            super::ifc_schema::IFCSPATIALSTRUCTUREELEMENTTYPE => "IfcSpatialStructureElementType".to_string(),
            super::ifc_schema::IFCSPHERE => "IfcSphere".to_string(),
            super::ifc_schema::IFCSTRUCTURALACTIVITY => "IfcStructuralActivity".to_string(),
            super::ifc_schema::IFCSTRUCTURALITEM => "IfcStructuralItem".to_string(),
            super::ifc_schema::IFCSTRUCTURALMEMBER => "IfcStructuralMember".to_string(),
            super::ifc_schema::IFCSTRUCTURALREACTION => "IfcStructuralReaction".to_string(),
            super::ifc_schema::IFCSTRUCTURALSURFACEMEMBER => "IfcStructuralSurfaceMember".to_string(),
            super::ifc_schema::IFCSTRUCTURALSURFACEMEMBERVARYING => "IfcStructuralSurfaceMemberVarying".to_string(),
            super::ifc_schema::IFCSTRUCTUREDDIMENSIONCALLOUT => "IfcStructuredDimensionCallout".to_string(),
            super::ifc_schema::IFCSURFACECURVESWEPTAREASOLID => "IfcSurfaceCurveSweptAreaSolid".to_string(),
            super::ifc_schema::IFCSURFACEOFLINEAREXTRUSION => "IfcSurfaceOfLinearExtrusion".to_string(),
            super::ifc_schema::IFCSURFACEOFREVOLUTION => "IfcSurfaceOfRevolution".to_string(),
            super::ifc_schema::IFCSYSTEMFURNITUREELEMENTTYPE => "IfcSystemFurnitureElementType".to_string(),
            super::ifc_schema::IFCTASK => "IfcTask".to_string(),
            super::ifc_schema::IFCTRANSPORTELEMENTTYPE => "IfcTransportElementType".to_string(),
            super::ifc_schema::IFCACTOR => "IfcActor".to_string(),
            super::ifc_schema::IFCANNOTATION => "IfcAnnotation".to_string(),
            super::ifc_schema::IFCASYMMETRICISHAPEPROFILEDEF => "IfcAsymmetricIShapeProfileDef".to_string(),
            super::ifc_schema::IFCBLOCK => "IfcBlock".to_string(),
            super::ifc_schema::IFCBOOLEANCLIPPINGRESULT => "IfcBooleanClippingResult".to_string(),
            super::ifc_schema::IFCBOUNDEDCURVE => "IfcBoundedCurve".to_string(),
            super::ifc_schema::IFCBUILDING => "IfcBuilding".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENTTYPE => "IfcBuildingElementType".to_string(),
            super::ifc_schema::IFCBUILDINGSTOREY => "IfcBuildingStorey".to_string(),
            super::ifc_schema::IFCCIRCLEHOLLOWPROFILEDEF => "IfcCircleHollowProfileDef".to_string(),
            super::ifc_schema::IFCCOLUMNTYPE => "IfcColumnType".to_string(),
            super::ifc_schema::IFCCOMPOSITECURVE => "IfcCompositeCurve".to_string(),
            super::ifc_schema::IFCCONIC => "IfcConic".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONRESOURCE => "IfcConstructionResource".to_string(),
            super::ifc_schema::IFCCONTROL => "IfcControl".to_string(),
            super::ifc_schema::IFCCOSTITEM => "IfcCostItem".to_string(),
            super::ifc_schema::IFCCOSTSCHEDULE => "IfcCostSchedule".to_string(),
            super::ifc_schema::IFCCOVERINGTYPE => "IfcCoveringType".to_string(),
            super::ifc_schema::IFCCREWRESOURCE => "IfcCrewResource".to_string(),
            super::ifc_schema::IFCCURTAINWALLTYPE => "IfcCurtainWallType".to_string(),
            super::ifc_schema::IFCDIMENSIONCURVEDIRECTEDCALLOUT => "IfcDimensionCurveDirectedCallout".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONELEMENTTYPE => "IfcDistributionElementType".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONFLOWELEMENTTYPE => "IfcDistributionFlowElementType".to_string(),
            super::ifc_schema::IFCELECTRICALBASEPROPERTIES => "IfcElectricalBaseProperties".to_string(),
            super::ifc_schema::IFCELEMENT => "IfcElement".to_string(),
            super::ifc_schema::IFCELEMENTASSEMBLY => "IfcElementAssembly".to_string(),
            super::ifc_schema::IFCELEMENTCOMPONENT => "IfcElementComponent".to_string(),
            super::ifc_schema::IFCELEMENTCOMPONENTTYPE => "IfcElementComponentType".to_string(),
            super::ifc_schema::IFCELLIPSE => "IfcEllipse".to_string(),
            super::ifc_schema::IFCENERGYCONVERSIONDEVICETYPE => "IfcEnergyConversionDeviceType".to_string(),
            super::ifc_schema::IFCEQUIPMENTELEMENT => "IfcEquipmentElement".to_string(),
            super::ifc_schema::IFCEQUIPMENTSTANDARD => "IfcEquipmentStandard".to_string(),
            super::ifc_schema::IFCEVAPORATIVECOOLERTYPE => "IfcEvaporativeCoolerType".to_string(),
            super::ifc_schema::IFCEVAPORATORTYPE => "IfcEvaporatorType".to_string(),
            super::ifc_schema::IFCFACETEDBREP => "IfcFacetedBrep".to_string(),
            super::ifc_schema::IFCFACETEDBREPWITHVOIDS => "IfcFacetedBrepWithVoids".to_string(),
            super::ifc_schema::IFCFASTENER => "IfcFastener".to_string(),
            super::ifc_schema::IFCFASTENERTYPE => "IfcFastenerType".to_string(),
            super::ifc_schema::IFCFEATUREELEMENT => "IfcFeatureElement".to_string(),
            super::ifc_schema::IFCFEATUREELEMENTADDITION => "IfcFeatureElementAddition".to_string(),
            super::ifc_schema::IFCFEATUREELEMENTSUBTRACTION => "IfcFeatureElementSubtraction".to_string(),
            super::ifc_schema::IFCFLOWCONTROLLERTYPE => "IfcFlowControllerType".to_string(),
            super::ifc_schema::IFCFLOWFITTINGTYPE => "IfcFlowFittingType".to_string(),
            super::ifc_schema::IFCFLOWMETERTYPE => "IfcFlowMeterType".to_string(),
            super::ifc_schema::IFCFLOWMOVINGDEVICETYPE => "IfcFlowMovingDeviceType".to_string(),
            super::ifc_schema::IFCFLOWSEGMENTTYPE => "IfcFlowSegmentType".to_string(),
            super::ifc_schema::IFCFLOWSTORAGEDEVICETYPE => "IfcFlowStorageDeviceType".to_string(),
            super::ifc_schema::IFCFLOWTERMINALTYPE => "IfcFlowTerminalType".to_string(),
            super::ifc_schema::IFCFLOWTREATMENTDEVICETYPE => "IfcFlowTreatmentDeviceType".to_string(),
            super::ifc_schema::IFCFURNISHINGELEMENT => "IfcFurnishingElement".to_string(),
            super::ifc_schema::IFCFURNITURESTANDARD => "IfcFurnitureStandard".to_string(),
            super::ifc_schema::IFCGASTERMINALTYPE => "IfcGasTerminalType".to_string(),
            super::ifc_schema::IFCGRID => "IfcGrid".to_string(),
            super::ifc_schema::IFCGROUP => "IfcGroup".to_string(),
            super::ifc_schema::IFCHEATEXCHANGERTYPE => "IfcHeatExchangerType".to_string(),
            super::ifc_schema::IFCHUMIDIFIERTYPE => "IfcHumidifierType".to_string(),
            super::ifc_schema::IFCINVENTORY => "IfcInventory".to_string(),
            super::ifc_schema::IFCJUNCTIONBOXTYPE => "IfcJunctionBoxType".to_string(),
            super::ifc_schema::IFCLABORRESOURCE => "IfcLaborResource".to_string(),
            super::ifc_schema::IFCLAMPTYPE => "IfcLampType".to_string(),
            super::ifc_schema::IFCLIGHTFIXTURETYPE => "IfcLightFixtureType".to_string(),
            super::ifc_schema::IFCLINEARDIMENSION => "IfcLinearDimension".to_string(),
            super::ifc_schema::IFCMECHANICALFASTENER => "IfcMechanicalFastener".to_string(),
            super::ifc_schema::IFCMECHANICALFASTENERTYPE => "IfcMechanicalFastenerType".to_string(),
            super::ifc_schema::IFCMEMBERTYPE => "IfcMemberType".to_string(),
            super::ifc_schema::IFCMOTORCONNECTIONTYPE => "IfcMotorConnectionType".to_string(),
            super::ifc_schema::IFCMOVE => "IfcMove".to_string(),
            super::ifc_schema::IFCOCCUPANT => "IfcOccupant".to_string(),
            super::ifc_schema::IFCOPENINGELEMENT => "IfcOpeningElement".to_string(),
            super::ifc_schema::IFCORDERACTION => "IfcOrderAction".to_string(),
            super::ifc_schema::IFCOUTLETTYPE => "IfcOutletType".to_string(),
            super::ifc_schema::IFCPERFORMANCEHISTORY => "IfcPerformanceHistory".to_string(),
            super::ifc_schema::IFCPERMIT => "IfcPermit".to_string(),
            super::ifc_schema::IFCPIPEFITTINGTYPE => "IfcPipeFittingType".to_string(),
            super::ifc_schema::IFCPIPESEGMENTTYPE => "IfcPipeSegmentType".to_string(),
            super::ifc_schema::IFCPLATETYPE => "IfcPlateType".to_string(),
            super::ifc_schema::IFCPOLYLINE => "IfcPolyline".to_string(),
            super::ifc_schema::IFCPORT => "IfcPort".to_string(),
            super::ifc_schema::IFCPROCEDURE => "IfcProcedure".to_string(),
            super::ifc_schema::IFCPROJECTORDER => "IfcProjectOrder".to_string(),
            super::ifc_schema::IFCPROJECTORDERRECORD => "IfcProjectOrderRecord".to_string(),
            super::ifc_schema::IFCPROJECTIONELEMENT => "IfcProjectionElement".to_string(),
            super::ifc_schema::IFCPROTECTIVEDEVICETYPE => "IfcProtectiveDeviceType".to_string(),
            super::ifc_schema::IFCPUMPTYPE => "IfcPumpType".to_string(),
            super::ifc_schema::IFCRADIUSDIMENSION => "IfcRadiusDimension".to_string(),
            super::ifc_schema::IFCRAILINGTYPE => "IfcRailingType".to_string(),
            super::ifc_schema::IFCRAMPFLIGHTTYPE => "IfcRampFlightType".to_string(),
            super::ifc_schema::IFCRELAGGREGATES => "IfcRelAggregates".to_string(),
            super::ifc_schema::IFCRELASSIGNSTASKS => "IfcRelAssignsTasks".to_string(),
            super::ifc_schema::IFCSANITARYTERMINALTYPE => "IfcSanitaryTerminalType".to_string(),
            super::ifc_schema::IFCSCHEDULETIMECONTROL => "IfcScheduleTimeControl".to_string(),
            super::ifc_schema::IFCSERVICELIFE => "IfcServiceLife".to_string(),
            super::ifc_schema::IFCSITE => "IfcSite".to_string(),
            super::ifc_schema::IFCSLABTYPE => "IfcSlabType".to_string(),
            super::ifc_schema::IFCSPACE => "IfcSpace".to_string(),
            super::ifc_schema::IFCSPACEHEATERTYPE => "IfcSpaceHeaterType".to_string(),
            super::ifc_schema::IFCSPACEPROGRAM => "IfcSpaceProgram".to_string(),
            super::ifc_schema::IFCSPACETYPE => "IfcSpaceType".to_string(),
            super::ifc_schema::IFCSTACKTERMINALTYPE => "IfcStackTerminalType".to_string(),
            super::ifc_schema::IFCSTAIRFLIGHTTYPE => "IfcStairFlightType".to_string(),
            super::ifc_schema::IFCSTRUCTURALACTION => "IfcStructuralAction".to_string(),
            super::ifc_schema::IFCSTRUCTURALCONNECTION => "IfcStructuralConnection".to_string(),
            super::ifc_schema::IFCSTRUCTURALCURVECONNECTION => "IfcStructuralCurveConnection".to_string(),
            super::ifc_schema::IFCSTRUCTURALCURVEMEMBER => "IfcStructuralCurveMember".to_string(),
            super::ifc_schema::IFCSTRUCTURALCURVEMEMBERVARYING => "IfcStructuralCurveMemberVarying".to_string(),
            super::ifc_schema::IFCSTRUCTURALLINEARACTION => "IfcStructuralLinearAction".to_string(),
            super::ifc_schema::IFCSTRUCTURALLINEARACTIONVARYING => "IfcStructuralLinearActionVarying".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADGROUP => "IfcStructuralLoadGroup".to_string(),
            super::ifc_schema::IFCSTRUCTURALPLANARACTION => "IfcStructuralPlanarAction".to_string(),
            super::ifc_schema::IFCSTRUCTURALPLANARACTIONVARYING => "IfcStructuralPlanarActionVarying".to_string(),
            super::ifc_schema::IFCSTRUCTURALPOINTACTION => "IfcStructuralPointAction".to_string(),
            super::ifc_schema::IFCSTRUCTURALPOINTCONNECTION => "IfcStructuralPointConnection".to_string(),
            super::ifc_schema::IFCSTRUCTURALPOINTREACTION => "IfcStructuralPointReaction".to_string(),
            super::ifc_schema::IFCSTRUCTURALRESULTGROUP => "IfcStructuralResultGroup".to_string(),
            super::ifc_schema::IFCSTRUCTURALSURFACECONNECTION => "IfcStructuralSurfaceConnection".to_string(),
            super::ifc_schema::IFCSUBCONTRACTRESOURCE => "IfcSubContractResource".to_string(),
            super::ifc_schema::IFCSWITCHINGDEVICETYPE => "IfcSwitchingDeviceType".to_string(),
            super::ifc_schema::IFCSYSTEM => "IfcSystem".to_string(),
            super::ifc_schema::IFCTANKTYPE => "IfcTankType".to_string(),
            super::ifc_schema::IFCTIMESERIESSCHEDULE => "IfcTimeSeriesSchedule".to_string(),
            super::ifc_schema::IFCTRANSFORMERTYPE => "IfcTransformerType".to_string(),
            super::ifc_schema::IFCTRANSPORTELEMENT => "IfcTransportElement".to_string(),
            super::ifc_schema::IFCTRIMMEDCURVE => "IfcTrimmedCurve".to_string(),
            super::ifc_schema::IFCTUBEBUNDLETYPE => "IfcTubeBundleType".to_string(),
            super::ifc_schema::IFCUNITARYEQUIPMENTTYPE => "IfcUnitaryEquipmentType".to_string(),
            super::ifc_schema::IFCVALVETYPE => "IfcValveType".to_string(),
            super::ifc_schema::IFCVIRTUALELEMENT => "IfcVirtualElement".to_string(),
            super::ifc_schema::IFCWALLTYPE => "IfcWallType".to_string(),
            super::ifc_schema::IFCWASTETERMINALTYPE => "IfcWasteTerminalType".to_string(),
            super::ifc_schema::IFCWORKCONTROL => "IfcWorkControl".to_string(),
            super::ifc_schema::IFCWORKPLAN => "IfcWorkPlan".to_string(),
            super::ifc_schema::IFCWORKSCHEDULE => "IfcWorkSchedule".to_string(),
            super::ifc_schema::IFCZONE => "IfcZone".to_string(),
            super::ifc_schema::IFC2DCOMPOSITECURVE => "Ifc2DCompositeCurve".to_string(),
            super::ifc_schema::IFCACTIONREQUEST => "IfcActionRequest".to_string(),
            super::ifc_schema::IFCAIRTERMINALBOXTYPE => "IfcAirTerminalBoxType".to_string(),
            super::ifc_schema::IFCAIRTERMINALTYPE => "IfcAirTerminalType".to_string(),
            super::ifc_schema::IFCAIRTOAIRHEATRECOVERYTYPE => "IfcAirToAirHeatRecoveryType".to_string(),
            super::ifc_schema::IFCANGULARDIMENSION => "IfcAngularDimension".to_string(),
            super::ifc_schema::IFCASSET => "IfcAsset".to_string(),
            super::ifc_schema::IFCBSPLINECURVE => "IfcBSplineCurve".to_string(),
            super::ifc_schema::IFCBEAMTYPE => "IfcBeamType".to_string(),
            super::ifc_schema::IFCBEZIERCURVE => "IfcBezierCurve".to_string(),
            super::ifc_schema::IFCBOILERTYPE => "IfcBoilerType".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENT => "IfcBuildingElement".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENTCOMPONENT => "IfcBuildingElementComponent".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENTPART => "IfcBuildingElementPart".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENTPROXY => "IfcBuildingElementProxy".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENTPROXYTYPE => "IfcBuildingElementProxyType".to_string(),
            super::ifc_schema::IFCCABLECARRIERFITTINGTYPE => "IfcCableCarrierFittingType".to_string(),
            super::ifc_schema::IFCCABLECARRIERSEGMENTTYPE => "IfcCableCarrierSegmentType".to_string(),
            super::ifc_schema::IFCCABLESEGMENTTYPE => "IfcCableSegmentType".to_string(),
            super::ifc_schema::IFCCHILLERTYPE => "IfcChillerType".to_string(),
            super::ifc_schema::IFCCIRCLE => "IfcCircle".to_string(),
            super::ifc_schema::IFCCOILTYPE => "IfcCoilType".to_string(),
            super::ifc_schema::IFCCOLUMN => "IfcColumn".to_string(),
            super::ifc_schema::IFCCOMPRESSORTYPE => "IfcCompressorType".to_string(),
            super::ifc_schema::IFCCONDENSERTYPE => "IfcCondenserType".to_string(),
            super::ifc_schema::IFCCONDITION => "IfcCondition".to_string(),
            super::ifc_schema::IFCCONDITIONCRITERION => "IfcConditionCriterion".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONEQUIPMENTRESOURCE => "IfcConstructionEquipmentResource".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONMATERIALRESOURCE => "IfcConstructionMaterialResource".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONPRODUCTRESOURCE => "IfcConstructionProductResource".to_string(),
            super::ifc_schema::IFCCOOLEDBEAMTYPE => "IfcCooledBeamType".to_string(),
            super::ifc_schema::IFCCOOLINGTOWERTYPE => "IfcCoolingTowerType".to_string(),
            super::ifc_schema::IFCCOVERING => "IfcCovering".to_string(),
            super::ifc_schema::IFCCURTAINWALL => "IfcCurtainWall".to_string(),
            super::ifc_schema::IFCDAMPERTYPE => "IfcDamperType".to_string(),
            super::ifc_schema::IFCDIAMETERDIMENSION => "IfcDiameterDimension".to_string(),
            super::ifc_schema::IFCDISCRETEACCESSORY => "IfcDiscreteAccessory".to_string(),
            super::ifc_schema::IFCDISCRETEACCESSORYTYPE => "IfcDiscreteAccessoryType".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONCHAMBERELEMENTTYPE => "IfcDistributionChamberElementType".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONCONTROLELEMENTTYPE => "IfcDistributionControlElementType".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONELEMENT => "IfcDistributionElement".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONFLOWELEMENT => "IfcDistributionFlowElement".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONPORT => "IfcDistributionPort".to_string(),
            super::ifc_schema::IFCDOOR => "IfcDoor".to_string(),
            super::ifc_schema::IFCDUCTFITTINGTYPE => "IfcDuctFittingType".to_string(),
            super::ifc_schema::IFCDUCTSEGMENTTYPE => "IfcDuctSegmentType".to_string(),
            super::ifc_schema::IFCDUCTSILENCERTYPE => "IfcDuctSilencerType".to_string(),
            super::ifc_schema::IFCEDGEFEATURE => "IfcEdgeFeature".to_string(),
            super::ifc_schema::IFCELECTRICAPPLIANCETYPE => "IfcElectricApplianceType".to_string(),
            super::ifc_schema::IFCELECTRICFLOWSTORAGEDEVICETYPE => "IfcElectricFlowStorageDeviceType".to_string(),
            super::ifc_schema::IFCELECTRICGENERATORTYPE => "IfcElectricGeneratorType".to_string(),
            super::ifc_schema::IFCELECTRICHEATERTYPE => "IfcElectricHeaterType".to_string(),
            super::ifc_schema::IFCELECTRICMOTORTYPE => "IfcElectricMotorType".to_string(),
            super::ifc_schema::IFCELECTRICTIMECONTROLTYPE => "IfcElectricTimeControlType".to_string(),
            super::ifc_schema::IFCELECTRICALCIRCUIT => "IfcElectricalCircuit".to_string(),
            super::ifc_schema::IFCELECTRICALELEMENT => "IfcElectricalElement".to_string(),
            super::ifc_schema::IFCENERGYCONVERSIONDEVICE => "IfcEnergyConversionDevice".to_string(),
            super::ifc_schema::IFCFANTYPE => "IfcFanType".to_string(),
            super::ifc_schema::IFCFILTERTYPE => "IfcFilterType".to_string(),
            super::ifc_schema::IFCFIRESUPPRESSIONTERMINALTYPE => "IfcFireSuppressionTerminalType".to_string(),
            super::ifc_schema::IFCFLOWCONTROLLER => "IfcFlowController".to_string(),
            super::ifc_schema::IFCFLOWFITTING => "IfcFlowFitting".to_string(),
            super::ifc_schema::IFCFLOWINSTRUMENTTYPE => "IfcFlowInstrumentType".to_string(),
            super::ifc_schema::IFCFLOWMOVINGDEVICE => "IfcFlowMovingDevice".to_string(),
            super::ifc_schema::IFCFLOWSEGMENT => "IfcFlowSegment".to_string(),
            super::ifc_schema::IFCFLOWSTORAGEDEVICE => "IfcFlowStorageDevice".to_string(),
            super::ifc_schema::IFCFLOWTERMINAL => "IfcFlowTerminal".to_string(),
            super::ifc_schema::IFCFLOWTREATMENTDEVICE => "IfcFlowTreatmentDevice".to_string(),
            super::ifc_schema::IFCFOOTING => "IfcFooting".to_string(),
            super::ifc_schema::IFCMEMBER => "IfcMember".to_string(),
            super::ifc_schema::IFCPILE => "IfcPile".to_string(),
            super::ifc_schema::IFCPLATE => "IfcPlate".to_string(),
            super::ifc_schema::IFCRAILING => "IfcRailing".to_string(),
            super::ifc_schema::IFCRAMP => "IfcRamp".to_string(),
            super::ifc_schema::IFCRAMPFLIGHT => "IfcRampFlight".to_string(),
            super::ifc_schema::IFCRATIONALBEZIERCURVE => "IfcRationalBezierCurve".to_string(),
            super::ifc_schema::IFCREINFORCINGELEMENT => "IfcReinforcingElement".to_string(),
            super::ifc_schema::IFCREINFORCINGMESH => "IfcReinforcingMesh".to_string(),
            super::ifc_schema::IFCROOF => "IfcRoof".to_string(),
            super::ifc_schema::IFCROUNDEDEDGEFEATURE => "IfcRoundedEdgeFeature".to_string(),
            super::ifc_schema::IFCSENSORTYPE => "IfcSensorType".to_string(),
            super::ifc_schema::IFCSLAB => "IfcSlab".to_string(),
            super::ifc_schema::IFCSTAIR => "IfcStair".to_string(),
            super::ifc_schema::IFCSTAIRFLIGHT => "IfcStairFlight".to_string(),
            super::ifc_schema::IFCSTRUCTURALANALYSISMODEL => "IfcStructuralAnalysisModel".to_string(),
            super::ifc_schema::IFCTENDON => "IfcTendon".to_string(),
            super::ifc_schema::IFCTENDONANCHOR => "IfcTendonAnchor".to_string(),
            super::ifc_schema::IFCVIBRATIONISOLATORTYPE => "IfcVibrationIsolatorType".to_string(),
            super::ifc_schema::IFCWALL => "IfcWall".to_string(),
            super::ifc_schema::IFCWALLSTANDARDCASE => "IfcWallStandardCase".to_string(),
            super::ifc_schema::IFCWINDOW => "IfcWindow".to_string(),
            super::ifc_schema::IFCACTUATORTYPE => "IfcActuatorType".to_string(),
            super::ifc_schema::IFCALARMTYPE => "IfcAlarmType".to_string(),
            super::ifc_schema::IFCBEAM => "IfcBeam".to_string(),
            super::ifc_schema::IFCCHAMFEREDGEFEATURE => "IfcChamferEdgeFeature".to_string(),
            super::ifc_schema::IFCCONTROLLERTYPE => "IfcControllerType".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONCHAMBERELEMENT => "IfcDistributionChamberElement".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONCONTROLELEMENT => "IfcDistributionControlElement".to_string(),
            super::ifc_schema::IFCELECTRICDISTRIBUTIONPOINT => "IfcElectricDistributionPoint".to_string(),
            super::ifc_schema::IFCREINFORCINGBAR => "IfcReinforcingBar".to_string(),
            super::ifc_schema::IFCCONNECTIONVOLUMEGEOMETRY => "IfcConnectionVolumeGeometry".to_string(),
            super::ifc_schema::IFCCOORDINATEOPERATION => "IfcCoordinateOperation".to_string(),
            super::ifc_schema::IFCCOORDINATEREFERENCESYSTEM => "IfcCoordinateReferenceSystem".to_string(),
            super::ifc_schema::IFCEXTERNALINFORMATION => "IfcExternalInformation".to_string(),
            super::ifc_schema::IFCMAPCONVERSION => "IfcMapConversion".to_string(),
            super::ifc_schema::IFCMATERIALDEFINITION => "IfcMaterialDefinition".to_string(),
            super::ifc_schema::IFCMATERIALLAYERWITHOFFSETS => "IfcMaterialLayerWithOffsets".to_string(),
            super::ifc_schema::IFCMATERIALPROFILE => "IfcMaterialProfile".to_string(),
            super::ifc_schema::IFCMATERIALPROFILESET => "IfcMaterialProfileSet".to_string(),
            super::ifc_schema::IFCMATERIALPROFILEWITHOFFSETS => "IfcMaterialProfileWithOffsets".to_string(),
            super::ifc_schema::IFCMATERIALUSAGEDEFINITION => "IfcMaterialUsageDefinition".to_string(),
            super::ifc_schema::IFCPRESENTATIONITEM => "IfcPresentationItem".to_string(),
            super::ifc_schema::IFCPROJECTEDCRS => "IfcProjectedCRS".to_string(),
            super::ifc_schema::IFCPROPERTYABSTRACTION => "IfcPropertyAbstraction".to_string(),
            super::ifc_schema::IFCRECURRENCEPATTERN => "IfcRecurrencePattern".to_string(),
            super::ifc_schema::IFCREFERENCE => "IfcReference".to_string(),
            super::ifc_schema::IFCRESOURCELEVELRELATIONSHIP => "IfcResourceLevelRelationship".to_string(),
            super::ifc_schema::IFCSCHEDULINGTIME => "IfcSchedulingTime".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADCONFIGURATION => "IfcStructuralLoadConfiguration".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADORRESULT => "IfcStructuralLoadOrResult".to_string(),
            super::ifc_schema::IFCSURFACEREINFORCEMENTAREA => "IfcSurfaceReinforcementArea".to_string(),
            super::ifc_schema::IFCTABLECOLUMN => "IfcTableColumn".to_string(),
            super::ifc_schema::IFCTASKTIME => "IfcTaskTime".to_string(),
            super::ifc_schema::IFCTASKTIMERECURRING => "IfcTaskTimeRecurring".to_string(),
            super::ifc_schema::IFCTEXTUREVERTEXLIST => "IfcTextureVertexList".to_string(),
            super::ifc_schema::IFCTIMEPERIOD => "IfcTimePeriod".to_string(),
            super::ifc_schema::IFCWORKTIME => "IfcWorkTime".to_string(),
            super::ifc_schema::IFCCOLOURRGBLIST => "IfcColourRgbList".to_string(),
            super::ifc_schema::IFCCONVERSIONBASEDUNITWITHOFFSET => "IfcConversionBasedUnitWithOffset".to_string(),
            super::ifc_schema::IFCEVENTTIME => "IfcEventTime".to_string(),
            super::ifc_schema::IFCEXTENDEDPROPERTIES => "IfcExtendedProperties".to_string(),
            super::ifc_schema::IFCEXTERNALREFERENCERELATIONSHIP => "IfcExternalReferenceRelationship".to_string(),
            super::ifc_schema::IFCINDEXEDCOLOURMAP => "IfcIndexedColourMap".to_string(),
            super::ifc_schema::IFCINDEXEDTEXTUREMAP => "IfcIndexedTextureMap".to_string(),
            super::ifc_schema::IFCINDEXEDTRIANGLETEXTUREMAP => "IfcIndexedTriangleTextureMap".to_string(),
            super::ifc_schema::IFCLAGTIME => "IfcLagTime".to_string(),
            super::ifc_schema::IFCMATERIALCONSTITUENT => "IfcMaterialConstituent".to_string(),
            super::ifc_schema::IFCMATERIALCONSTITUENTSET => "IfcMaterialConstituentSet".to_string(),
            super::ifc_schema::IFCMATERIALPROFILESETUSAGE => "IfcMaterialProfileSetUsage".to_string(),
            super::ifc_schema::IFCMATERIALPROFILESETUSAGETAPERING => "IfcMaterialProfileSetUsageTapering".to_string(),
            super::ifc_schema::IFCMATERIALRELATIONSHIP => "IfcMaterialRelationship".to_string(),
            super::ifc_schema::IFCMIRROREDPROFILEDEF => "IfcMirroredProfileDef".to_string(),
            super::ifc_schema::IFCPREDEFINEDPROPERTIES => "IfcPreDefinedProperties".to_string(),
            super::ifc_schema::IFCPROPERTYTEMPLATEDEFINITION => "IfcPropertyTemplateDefinition".to_string(),
            super::ifc_schema::IFCQUANTITYSET => "IfcQuantitySet".to_string(),
            super::ifc_schema::IFCRESOURCEAPPROVALRELATIONSHIP => "IfcResourceApprovalRelationship".to_string(),
            super::ifc_schema::IFCRESOURCECONSTRAINTRELATIONSHIP => "IfcResourceConstraintRelationship".to_string(),
            super::ifc_schema::IFCRESOURCETIME => "IfcResourceTime".to_string(),
            super::ifc_schema::IFCSWEPTDISKSOLIDPOLYGONAL => "IfcSweptDiskSolidPolygonal".to_string(),
            super::ifc_schema::IFCTESSELLATEDITEM => "IfcTessellatedItem".to_string(),
            super::ifc_schema::IFCTYPEPROCESS => "IfcTypeProcess".to_string(),
            super::ifc_schema::IFCTYPERESOURCE => "IfcTypeResource".to_string(),
            super::ifc_schema::IFCADVANCEDFACE => "IfcAdvancedFace".to_string(),
            super::ifc_schema::IFCCARTESIANPOINTLIST => "IfcCartesianPointList".to_string(),
            super::ifc_schema::IFCCARTESIANPOINTLIST2D => "IfcCartesianPointList2D".to_string(),
            super::ifc_schema::IFCCARTESIANPOINTLIST3D => "IfcCartesianPointList3D".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONRESOURCETYPE => "IfcConstructionResourceType".to_string(),
            super::ifc_schema::IFCCONTEXT => "IfcContext".to_string(),
            super::ifc_schema::IFCCREWRESOURCETYPE => "IfcCrewResourceType".to_string(),
            super::ifc_schema::IFCCURVEBOUNDEDSURFACE => "IfcCurveBoundedSurface".to_string(),
            super::ifc_schema::IFCEVENTTYPE => "IfcEventType".to_string(),
            super::ifc_schema::IFCEXTRUDEDAREASOLIDTAPERED => "IfcExtrudedAreaSolidTapered".to_string(),
            super::ifc_schema::IFCFIXEDREFERENCESWEPTAREASOLID => "IfcFixedReferenceSweptAreaSolid".to_string(),
            super::ifc_schema::IFCGEOGRAPHICELEMENTTYPE => "IfcGeographicElementType".to_string(),
            super::ifc_schema::IFCINDEXEDPOLYGONALFACE => "IfcIndexedPolygonalFace".to_string(),
            super::ifc_schema::IFCINDEXEDPOLYGONALFACEWITHVOIDS => "IfcIndexedPolygonalFaceWithVoids".to_string(),
            super::ifc_schema::IFCLABORRESOURCETYPE => "IfcLaborResourceType".to_string(),
            super::ifc_schema::IFCPCURVE => "IfcPcurve".to_string(),
            super::ifc_schema::IFCPREDEFINEDPROPERTYSET => "IfcPreDefinedPropertySet".to_string(),
            super::ifc_schema::IFCPROCEDURETYPE => "IfcProcedureType".to_string(),
            super::ifc_schema::IFCPROJECTLIBRARY => "IfcProjectLibrary".to_string(),
            super::ifc_schema::IFCPROPERTYSETTEMPLATE => "IfcPropertySetTemplate".to_string(),
            super::ifc_schema::IFCPROPERTYTEMPLATE => "IfcPropertyTemplate".to_string(),
            super::ifc_schema::IFCRELASSIGNSTOGROUPBYFACTOR => "IfcRelAssignsToGroupByFactor".to_string(),
            super::ifc_schema::IFCRELDECLARES => "IfcRelDeclares".to_string(),
            super::ifc_schema::IFCRELDEFINESBYOBJECT => "IfcRelDefinesByObject".to_string(),
            super::ifc_schema::IFCRELDEFINESBYTEMPLATE => "IfcRelDefinesByTemplate".to_string(),
            super::ifc_schema::IFCRELINTERFERESELEMENTS => "IfcRelInterferesElements".to_string(),
            super::ifc_schema::IFCRELSPACEBOUNDARY1STLEVEL => "IfcRelSpaceBoundary1stLevel".to_string(),
            super::ifc_schema::IFCRELSPACEBOUNDARY2NDLEVEL => "IfcRelSpaceBoundary2ndLevel".to_string(),
            super::ifc_schema::IFCREPARAMETRISEDCOMPOSITECURVESEGMENT => "IfcReparametrisedCompositeCurveSegment".to_string(),
            super::ifc_schema::IFCREVOLVEDAREASOLIDTAPERED => "IfcRevolvedAreaSolidTapered".to_string(),
            super::ifc_schema::IFCSIMPLEPROPERTYTEMPLATE => "IfcSimplePropertyTemplate".to_string(),
            super::ifc_schema::IFCSPATIALELEMENT => "IfcSpatialElement".to_string(),
            super::ifc_schema::IFCSPATIALELEMENTTYPE => "IfcSpatialElementType".to_string(),
            super::ifc_schema::IFCSPATIALZONE => "IfcSpatialZone".to_string(),
            super::ifc_schema::IFCSPATIALZONETYPE => "IfcSpatialZoneType".to_string(),
            super::ifc_schema::IFCSPHERICALSURFACE => "IfcSphericalSurface".to_string(),
            super::ifc_schema::IFCSTRUCTURALSURFACEREACTION => "IfcStructuralSurfaceReaction".to_string(),
            super::ifc_schema::IFCSUBCONTRACTRESOURCETYPE => "IfcSubContractResourceType".to_string(),
            super::ifc_schema::IFCSURFACECURVE => "IfcSurfaceCurve".to_string(),
            super::ifc_schema::IFCTASKTYPE => "IfcTaskType".to_string(),
            super::ifc_schema::IFCTESSELLATEDFACESET => "IfcTessellatedFaceSet".to_string(),
            super::ifc_schema::IFCTOROIDALSURFACE => "IfcToroidalSurface".to_string(),
            super::ifc_schema::IFCTRIANGULATEDFACESET => "IfcTriangulatedFaceSet".to_string(),
            super::ifc_schema::IFCADVANCEDBREP => "IfcAdvancedBrep".to_string(),
            super::ifc_schema::IFCADVANCEDBREPWITHVOIDS => "IfcAdvancedBrepWithVoids".to_string(),
            super::ifc_schema::IFCBSPLINESURFACE => "IfcBSplineSurface".to_string(),
            super::ifc_schema::IFCBSPLINESURFACEWITHKNOTS => "IfcBSplineSurfaceWithKnots".to_string(),
            super::ifc_schema::IFCCHIMNEYTYPE => "IfcChimneyType".to_string(),
            super::ifc_schema::IFCCIVILELEMENTTYPE => "IfcCivilElementType".to_string(),
            super::ifc_schema::IFCCOMPLEXPROPERTYTEMPLATE => "IfcComplexPropertyTemplate".to_string(),
            super::ifc_schema::IFCCOMPOSITECURVEONSURFACE => "IfcCompositeCurveOnSurface".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONEQUIPMENTRESOURCETYPE => "IfcConstructionEquipmentResourceType".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONMATERIALRESOURCETYPE => "IfcConstructionMaterialResourceType".to_string(),
            super::ifc_schema::IFCCONSTRUCTIONPRODUCTRESOURCETYPE => "IfcConstructionProductResourceType".to_string(),
            super::ifc_schema::IFCCYLINDRICALSURFACE => "IfcCylindricalSurface".to_string(),
            super::ifc_schema::IFCDOORTYPE => "IfcDoorType".to_string(),
            super::ifc_schema::IFCELEMENTASSEMBLYTYPE => "IfcElementAssemblyType".to_string(),
            super::ifc_schema::IFCENGINETYPE => "IfcEngineType".to_string(),
            super::ifc_schema::IFCEVENT => "IfcEvent".to_string(),
            super::ifc_schema::IFCEXTERNALSPATIALSTRUCTUREELEMENT => "IfcExternalSpatialStructureElement".to_string(),
            super::ifc_schema::IFCFOOTINGTYPE => "IfcFootingType".to_string(),
            super::ifc_schema::IFCFURNITURE => "IfcFurniture".to_string(),
            super::ifc_schema::IFCGEOGRAPHICELEMENT => "IfcGeographicElement".to_string(),
            super::ifc_schema::IFCINDEXEDPOLYCURVE => "IfcIndexedPolyCurve".to_string(),
            super::ifc_schema::IFCINTERCEPTORTYPE => "IfcInterceptorType".to_string(),
            super::ifc_schema::IFCINTERSECTIONCURVE => "IfcIntersectionCurve".to_string(),
            super::ifc_schema::IFCMEDICALDEVICETYPE => "IfcMedicalDeviceType".to_string(),
            super::ifc_schema::IFCOPENINGSTANDARDCASE => "IfcOpeningStandardCase".to_string(),
            super::ifc_schema::IFCPILETYPE => "IfcPileType".to_string(),
            super::ifc_schema::IFCPOLYGONALFACESET => "IfcPolygonalFaceSet".to_string(),
            super::ifc_schema::IFCRAMPTYPE => "IfcRampType".to_string(),
            super::ifc_schema::IFCRATIONALBSPLINESURFACEWITHKNOTS => "IfcRationalBSplineSurfaceWithKnots".to_string(),
            super::ifc_schema::IFCREINFORCINGELEMENTTYPE => "IfcReinforcingElementType".to_string(),
            super::ifc_schema::IFCREINFORCINGMESHTYPE => "IfcReinforcingMeshType".to_string(),
            super::ifc_schema::IFCROOFTYPE => "IfcRoofType".to_string(),
            super::ifc_schema::IFCSEAMCURVE => "IfcSeamCurve".to_string(),
            super::ifc_schema::IFCSHADINGDEVICETYPE => "IfcShadingDeviceType".to_string(),
            super::ifc_schema::IFCSOLARDEVICETYPE => "IfcSolarDeviceType".to_string(),
            super::ifc_schema::IFCSTAIRTYPE => "IfcStairType".to_string(),
            super::ifc_schema::IFCSTRUCTURALCURVEACTION => "IfcStructuralCurveAction".to_string(),
            super::ifc_schema::IFCSTRUCTURALCURVEREACTION => "IfcStructuralCurveReaction".to_string(),
            super::ifc_schema::IFCSTRUCTURALSURFACEACTION => "IfcStructuralSurfaceAction".to_string(),
            super::ifc_schema::IFCSURFACEFEATURE => "IfcSurfaceFeature".to_string(),
            super::ifc_schema::IFCSYSTEMFURNITUREELEMENT => "IfcSystemFurnitureElement".to_string(),
            super::ifc_schema::IFCTENDONANCHORTYPE => "IfcTendonAnchorType".to_string(),
            super::ifc_schema::IFCTENDONTYPE => "IfcTendonType".to_string(),
            super::ifc_schema::IFCVIBRATIONISOLATOR => "IfcVibrationIsolator".to_string(),
            super::ifc_schema::IFCVOIDINGFEATURE => "IfcVoidingFeature".to_string(),
            super::ifc_schema::IFCWINDOWTYPE => "IfcWindowType".to_string(),
            super::ifc_schema::IFCWORKCALENDAR => "IfcWorkCalendar".to_string(),
            super::ifc_schema::IFCAUDIOVISUALAPPLIANCETYPE => "IfcAudioVisualApplianceType".to_string(),
            super::ifc_schema::IFCBSPLINECURVEWITHKNOTS => "IfcBSplineCurveWithKnots".to_string(),
            super::ifc_schema::IFCBOUNDARYCURVE => "IfcBoundaryCurve".to_string(),
            super::ifc_schema::IFCBUILDINGELEMENTPARTTYPE => "IfcBuildingElementPartType".to_string(),
            super::ifc_schema::IFCBUILDINGSYSTEM => "IfcBuildingSystem".to_string(),
            super::ifc_schema::IFCBURNERTYPE => "IfcBurnerType".to_string(),
            super::ifc_schema::IFCCABLEFITTINGTYPE => "IfcCableFittingType".to_string(),
            super::ifc_schema::IFCCHIMNEY => "IfcChimney".to_string(),
            super::ifc_schema::IFCCIVILELEMENT => "IfcCivilElement".to_string(),
            super::ifc_schema::IFCCOLUMNSTANDARDCASE => "IfcColumnStandardCase".to_string(),
            super::ifc_schema::IFCCOMMUNICATIONSAPPLIANCETYPE => "IfcCommunicationsApplianceType".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONSYSTEM => "IfcDistributionSystem".to_string(),
            super::ifc_schema::IFCDOORSTANDARDCASE => "IfcDoorStandardCase".to_string(),
            super::ifc_schema::IFCELECTRICDISTRIBUTIONBOARDTYPE => "IfcElectricDistributionBoardType".to_string(),
            super::ifc_schema::IFCENGINE => "IfcEngine".to_string(),
            super::ifc_schema::IFCEVAPORATIVECOOLER => "IfcEvaporativeCooler".to_string(),
            super::ifc_schema::IFCEVAPORATOR => "IfcEvaporator".to_string(),
            super::ifc_schema::IFCEXTERNALSPATIALELEMENT => "IfcExternalSpatialElement".to_string(),
            super::ifc_schema::IFCFLOWMETER => "IfcFlowMeter".to_string(),
            super::ifc_schema::IFCHEATEXCHANGER => "IfcHeatExchanger".to_string(),
            super::ifc_schema::IFCHUMIDIFIER => "IfcHumidifier".to_string(),
            super::ifc_schema::IFCINTERCEPTOR => "IfcInterceptor".to_string(),
            super::ifc_schema::IFCJUNCTIONBOX => "IfcJunctionBox".to_string(),
            super::ifc_schema::IFCLAMP => "IfcLamp".to_string(),
            super::ifc_schema::IFCLIGHTFIXTURE => "IfcLightFixture".to_string(),
            super::ifc_schema::IFCMEDICALDEVICE => "IfcMedicalDevice".to_string(),
            super::ifc_schema::IFCMEMBERSTANDARDCASE => "IfcMemberStandardCase".to_string(),
            super::ifc_schema::IFCMOTORCONNECTION => "IfcMotorConnection".to_string(),
            super::ifc_schema::IFCOUTERBOUNDARYCURVE => "IfcOuterBoundaryCurve".to_string(),
            super::ifc_schema::IFCOUTLET => "IfcOutlet".to_string(),
            super::ifc_schema::IFCPIPEFITTING => "IfcPipeFitting".to_string(),
            super::ifc_schema::IFCPIPESEGMENT => "IfcPipeSegment".to_string(),
            super::ifc_schema::IFCPLATESTANDARDCASE => "IfcPlateStandardCase".to_string(),
            super::ifc_schema::IFCPROTECTIVEDEVICE => "IfcProtectiveDevice".to_string(),
            super::ifc_schema::IFCPROTECTIVEDEVICETRIPPINGUNITTYPE => "IfcProtectiveDeviceTrippingUnitType".to_string(),
            super::ifc_schema::IFCPUMP => "IfcPump".to_string(),
            super::ifc_schema::IFCRATIONALBSPLINECURVEWITHKNOTS => "IfcRationalBSplineCurveWithKnots".to_string(),
            super::ifc_schema::IFCREINFORCINGBARTYPE => "IfcReinforcingBarType".to_string(),
            super::ifc_schema::IFCSANITARYTERMINAL => "IfcSanitaryTerminal".to_string(),
            super::ifc_schema::IFCSHADINGDEVICE => "IfcShadingDevice".to_string(),
            super::ifc_schema::IFCSLABELEMENTEDCASE => "IfcSlabElementedCase".to_string(),
            super::ifc_schema::IFCSLABSTANDARDCASE => "IfcSlabStandardCase".to_string(),
            super::ifc_schema::IFCSOLARDEVICE => "IfcSolarDevice".to_string(),
            super::ifc_schema::IFCSPACEHEATER => "IfcSpaceHeater".to_string(),
            super::ifc_schema::IFCSTACKTERMINAL => "IfcStackTerminal".to_string(),
            super::ifc_schema::IFCSTRUCTURALLOADCASE => "IfcStructuralLoadCase".to_string(),
            super::ifc_schema::IFCSWITCHINGDEVICE => "IfcSwitchingDevice".to_string(),
            super::ifc_schema::IFCTANK => "IfcTank".to_string(),
            super::ifc_schema::IFCTRANSFORMER => "IfcTransformer".to_string(),
            super::ifc_schema::IFCTUBEBUNDLE => "IfcTubeBundle".to_string(),
            super::ifc_schema::IFCUNITARYCONTROLELEMENTTYPE => "IfcUnitaryControlElementType".to_string(),
            super::ifc_schema::IFCUNITARYEQUIPMENT => "IfcUnitaryEquipment".to_string(),
            super::ifc_schema::IFCVALVE => "IfcValve".to_string(),
            super::ifc_schema::IFCWALLELEMENTEDCASE => "IfcWallElementedCase".to_string(),
            super::ifc_schema::IFCWASTETERMINAL => "IfcWasteTerminal".to_string(),
            super::ifc_schema::IFCWINDOWSTANDARDCASE => "IfcWindowStandardCase".to_string(),
            super::ifc_schema::IFCAIRTERMINAL => "IfcAirTerminal".to_string(),
            super::ifc_schema::IFCAIRTERMINALBOX => "IfcAirTerminalBox".to_string(),
            super::ifc_schema::IFCAIRTOAIRHEATRECOVERY => "IfcAirToAirHeatRecovery".to_string(),
            super::ifc_schema::IFCAUDIOVISUALAPPLIANCE => "IfcAudioVisualAppliance".to_string(),
            super::ifc_schema::IFCBEAMSTANDARDCASE => "IfcBeamStandardCase".to_string(),
            super::ifc_schema::IFCBOILER => "IfcBoiler".to_string(),
            super::ifc_schema::IFCBURNER => "IfcBurner".to_string(),
            super::ifc_schema::IFCCABLECARRIERFITTING => "IfcCableCarrierFitting".to_string(),
            super::ifc_schema::IFCCABLECARRIERSEGMENT => "IfcCableCarrierSegment".to_string(),
            super::ifc_schema::IFCCABLEFITTING => "IfcCableFitting".to_string(),
            super::ifc_schema::IFCCABLESEGMENT => "IfcCableSegment".to_string(),
            super::ifc_schema::IFCCHILLER => "IfcChiller".to_string(),
            super::ifc_schema::IFCCOIL => "IfcCoil".to_string(),
            super::ifc_schema::IFCCOMMUNICATIONSAPPLIANCE => "IfcCommunicationsAppliance".to_string(),
            super::ifc_schema::IFCCOMPRESSOR => "IfcCompressor".to_string(),
            super::ifc_schema::IFCCONDENSER => "IfcCondenser".to_string(),
            super::ifc_schema::IFCCOOLEDBEAM => "IfcCooledBeam".to_string(),
            super::ifc_schema::IFCCOOLINGTOWER => "IfcCoolingTower".to_string(),
            super::ifc_schema::IFCDAMPER => "IfcDamper".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONCIRCUIT => "IfcDistributionCircuit".to_string(),
            super::ifc_schema::IFCDUCTFITTING => "IfcDuctFitting".to_string(),
            super::ifc_schema::IFCDUCTSEGMENT => "IfcDuctSegment".to_string(),
            super::ifc_schema::IFCDUCTSILENCER => "IfcDuctSilencer".to_string(),
            super::ifc_schema::IFCELECTRICAPPLIANCE => "IfcElectricAppliance".to_string(),
            super::ifc_schema::IFCELECTRICDISTRIBUTIONBOARD => "IfcElectricDistributionBoard".to_string(),
            super::ifc_schema::IFCELECTRICFLOWSTORAGEDEVICE => "IfcElectricFlowStorageDevice".to_string(),
            super::ifc_schema::IFCELECTRICGENERATOR => "IfcElectricGenerator".to_string(),
            super::ifc_schema::IFCELECTRICMOTOR => "IfcElectricMotor".to_string(),
            super::ifc_schema::IFCELECTRICTIMECONTROL => "IfcElectricTimeControl".to_string(),
            super::ifc_schema::IFCFAN => "IfcFan".to_string(),
            super::ifc_schema::IFCFILTER => "IfcFilter".to_string(),
            super::ifc_schema::IFCFIRESUPPRESSIONTERMINAL => "IfcFireSuppressionTerminal".to_string(),
            super::ifc_schema::IFCFLOWINSTRUMENT => "IfcFlowInstrument".to_string(),
            super::ifc_schema::IFCPROTECTIVEDEVICETRIPPINGUNIT => "IfcProtectiveDeviceTrippingUnit".to_string(),
            super::ifc_schema::IFCSENSOR => "IfcSensor".to_string(),
            super::ifc_schema::IFCUNITARYCONTROLELEMENT => "IfcUnitaryControlElement".to_string(),
            super::ifc_schema::IFCACTUATOR => "IfcActuator".to_string(),
            super::ifc_schema::IFCALARM => "IfcAlarm".to_string(),
            super::ifc_schema::IFCCONTROLLER => "IfcController".to_string(),
            super::ifc_schema::IFCALIGNMENTPARAMETERSEGMENT => "IfcAlignmentParameterSegment".to_string(),
            super::ifc_schema::IFCALIGNMENTVERTICALSEGMENT => "IfcAlignmentVerticalSegment".to_string(),
            super::ifc_schema::IFCGEOGRAPHICCRS => "IfcGeographicCRS".to_string(),
            super::ifc_schema::IFCMAPCONVERSIONSCALED => "IfcMapConversionScaled".to_string(),
            super::ifc_schema::IFCQUANTITYNUMBER => "IfcQuantityNumber".to_string(),
            super::ifc_schema::IFCRIGIDOPERATION => "IfcRigidOperation".to_string(),
            super::ifc_schema::IFCTEXTURECOORDINATEINDICES => "IfcTextureCoordinateIndices".to_string(),
            super::ifc_schema::IFCTEXTURECOORDINATEINDICESWITHVOIDS => "IfcTextureCoordinateIndicesWithVoids".to_string(),
            super::ifc_schema::IFCWELLKNOWNTEXT => "IfcWellKnownText".to_string(),
            super::ifc_schema::IFCALIGNMENTCANTSEGMENT => "IfcAlignmentCantSegment".to_string(),
            super::ifc_schema::IFCALIGNMENTHORIZONTALSEGMENT => "IfcAlignmentHorizontalSegment".to_string(),
            super::ifc_schema::IFCLINEARPLACEMENT => "IfcLinearPlacement".to_string(),
            super::ifc_schema::IFCOPENCROSSPROFILEDEF => "IfcOpenCrossProfileDef".to_string(),
            super::ifc_schema::IFCPOINTBYDISTANCEEXPRESSION => "IfcPointByDistanceExpression".to_string(),
            super::ifc_schema::IFCSEGMENT => "IfcSegment".to_string(),
            super::ifc_schema::IFCAXIS2PLACEMENTLINEAR => "IfcAxis2PlacementLinear".to_string(),
            super::ifc_schema::IFCCURVESEGMENT => "IfcCurveSegment".to_string(),
            super::ifc_schema::IFCDIRECTRIXCURVESWEPTAREASOLID => "IfcDirectrixCurveSweptAreaSolid".to_string(),
            super::ifc_schema::IFCINDEXEDPOLYGONALTEXTUREMAP => "IfcIndexedPolygonalTextureMap".to_string(),
            super::ifc_schema::IFCOFFSETCURVE => "IfcOffsetCurve".to_string(),
            super::ifc_schema::IFCOFFSETCURVEBYDISTANCES => "IfcOffsetCurveByDistances".to_string(),
            super::ifc_schema::IFCPOLYNOMIALCURVE => "IfcPolynomialCurve".to_string(),
            super::ifc_schema::IFCRELASSOCIATESPROFILEDEF => "IfcRelAssociatesProfileDef".to_string(),
            super::ifc_schema::IFCRELPOSITIONS => "IfcRelPositions".to_string(),
            super::ifc_schema::IFCSECTIONEDSOLID => "IfcSectionedSolid".to_string(),
            super::ifc_schema::IFCSECTIONEDSOLIDHORIZONTAL => "IfcSectionedSolidHorizontal".to_string(),
            super::ifc_schema::IFCSECTIONEDSURFACE => "IfcSectionedSurface".to_string(),
            super::ifc_schema::IFCSPIRAL => "IfcSpiral".to_string(),
            super::ifc_schema::IFCTHIRDORDERPOLYNOMIALSPIRAL => "IfcThirdOrderPolynomialSpiral".to_string(),
            super::ifc_schema::IFCTRANSPORTATIONDEVICETYPE => "IfcTransportationDeviceType".to_string(),
            super::ifc_schema::IFCTRIANGULATEDIRREGULARNETWORK => "IfcTriangulatedIrregularNetwork".to_string(),
            super::ifc_schema::IFCVEHICLETYPE => "IfcVehicleType".to_string(),
            super::ifc_schema::IFCBUILTELEMENTTYPE => "IfcBuiltElementType".to_string(),
            super::ifc_schema::IFCCLOTHOID => "IfcClothoid".to_string(),
            super::ifc_schema::IFCCOSINESPIRAL => "IfcCosineSpiral".to_string(),
            super::ifc_schema::IFCCOURSETYPE => "IfcCourseType".to_string(),
            super::ifc_schema::IFCDEEPFOUNDATIONTYPE => "IfcDeepFoundationType".to_string(),
            super::ifc_schema::IFCDIRECTRIXDERIVEDREFERENCESWEPTAREASOLID => "IfcDirectrixDerivedReferenceSweptAreaSolid".to_string(),
            super::ifc_schema::IFCFACILITY => "IfcFacility".to_string(),
            super::ifc_schema::IFCFACILITYPART => "IfcFacilityPart".to_string(),
            super::ifc_schema::IFCFACILITYPARTCOMMON => "IfcFacilityPartCommon".to_string(),
            super::ifc_schema::IFCGEOTECHNICALELEMENT => "IfcGeotechnicalElement".to_string(),
            super::ifc_schema::IFCGEOTECHNICALSTRATUM => "IfcGeotechnicalStratum".to_string(),
            super::ifc_schema::IFCGRADIENTCURVE => "IfcGradientCurve".to_string(),
            super::ifc_schema::IFCIMPACTPROTECTIONDEVICE => "IfcImpactProtectionDevice".to_string(),
            super::ifc_schema::IFCIMPACTPROTECTIONDEVICETYPE => "IfcImpactProtectionDeviceType".to_string(),
            super::ifc_schema::IFCKERBTYPE => "IfcKerbType".to_string(),
            super::ifc_schema::IFCLINEARELEMENT => "IfcLinearElement".to_string(),
            super::ifc_schema::IFCLIQUIDTERMINALTYPE => "IfcLiquidTerminalType".to_string(),
            super::ifc_schema::IFCMARINEFACILITY => "IfcMarineFacility".to_string(),
            super::ifc_schema::IFCMARINEPART => "IfcMarinePart".to_string(),
            super::ifc_schema::IFCMOBILETELECOMMUNICATIONSAPPLIANCETYPE => "IfcMobileTelecommunicationsApplianceType".to_string(),
            super::ifc_schema::IFCMOORINGDEVICETYPE => "IfcMooringDeviceType".to_string(),
            super::ifc_schema::IFCNAVIGATIONELEMENTTYPE => "IfcNavigationElementType".to_string(),
            super::ifc_schema::IFCPAVEMENTTYPE => "IfcPavementType".to_string(),
            super::ifc_schema::IFCPOSITIONINGELEMENT => "IfcPositioningElement".to_string(),
            super::ifc_schema::IFCRAILTYPE => "IfcRailType".to_string(),
            super::ifc_schema::IFCRAILWAY => "IfcRailway".to_string(),
            super::ifc_schema::IFCRAILWAYPART => "IfcRailwayPart".to_string(),
            super::ifc_schema::IFCREFERENT => "IfcReferent".to_string(),
            super::ifc_schema::IFCRELADHERESTOELEMENT => "IfcRelAdheresToElement".to_string(),
            super::ifc_schema::IFCROAD => "IfcRoad".to_string(),
            super::ifc_schema::IFCROADPART => "IfcRoadPart".to_string(),
            super::ifc_schema::IFCSECONDORDERPOLYNOMIALSPIRAL => "IfcSecondOrderPolynomialSpiral".to_string(),
            super::ifc_schema::IFCSEGMENTEDREFERENCECURVE => "IfcSegmentedReferenceCurve".to_string(),
            super::ifc_schema::IFCSEVENTHORDERPOLYNOMIALSPIRAL => "IfcSeventhOrderPolynomialSpiral".to_string(),
            super::ifc_schema::IFCSIGN => "IfcSign".to_string(),
            super::ifc_schema::IFCSIGNTYPE => "IfcSignType".to_string(),
            super::ifc_schema::IFCSIGNALTYPE => "IfcSignalType".to_string(),
            super::ifc_schema::IFCSINESPIRAL => "IfcSineSpiral".to_string(),
            super::ifc_schema::IFCTENDONCONDUIT => "IfcTendonConduit".to_string(),
            super::ifc_schema::IFCTENDONCONDUITTYPE => "IfcTendonConduitType".to_string(),
            super::ifc_schema::IFCTRACKELEMENTTYPE => "IfcTrackElementType".to_string(),
            super::ifc_schema::IFCTRANSPORTATIONDEVICE => "IfcTransportationDevice".to_string(),
            super::ifc_schema::IFCVEHICLE => "IfcVehicle".to_string(),
            super::ifc_schema::IFCVIBRATIONDAMPER => "IfcVibrationDamper".to_string(),
            super::ifc_schema::IFCVIBRATIONDAMPERTYPE => "IfcVibrationDamperType".to_string(),
            super::ifc_schema::IFCALIGNMENTCANT => "IfcAlignmentCant".to_string(),
            super::ifc_schema::IFCALIGNMENTHORIZONTAL => "IfcAlignmentHorizontal".to_string(),
            super::ifc_schema::IFCALIGNMENTSEGMENT => "IfcAlignmentSegment".to_string(),
            super::ifc_schema::IFCALIGNMENTVERTICAL => "IfcAlignmentVertical".to_string(),
            super::ifc_schema::IFCBEARINGTYPE => "IfcBearingType".to_string(),
            super::ifc_schema::IFCBRIDGE => "IfcBridge".to_string(),
            super::ifc_schema::IFCBRIDGEPART => "IfcBridgePart".to_string(),
            super::ifc_schema::IFCBUILTELEMENT => "IfcBuiltElement".to_string(),
            super::ifc_schema::IFCBUILTSYSTEM => "IfcBuiltSystem".to_string(),
            super::ifc_schema::IFCCAISSONFOUNDATIONTYPE => "IfcCaissonFoundationType".to_string(),
            super::ifc_schema::IFCCONVEYORSEGMENTTYPE => "IfcConveyorSegmentType".to_string(),
            super::ifc_schema::IFCCOURSE => "IfcCourse".to_string(),
            super::ifc_schema::IFCDEEPFOUNDATION => "IfcDeepFoundation".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONBOARDTYPE => "IfcDistributionBoardType".to_string(),
            super::ifc_schema::IFCEARTHWORKSCUT => "IfcEarthworksCut".to_string(),
            super::ifc_schema::IFCEARTHWORKSELEMENT => "IfcEarthworksElement".to_string(),
            super::ifc_schema::IFCEARTHWORKSFILL => "IfcEarthworksFill".to_string(),
            super::ifc_schema::IFCELECTRICFLOWTREATMENTDEVICETYPE => "IfcElectricFlowTreatmentDeviceType".to_string(),
            super::ifc_schema::IFCGEOTECHNICALASSEMBLY => "IfcGeotechnicalAssembly".to_string(),
            super::ifc_schema::IFCKERB => "IfcKerb".to_string(),
            super::ifc_schema::IFCLINEARPOSITIONINGELEMENT => "IfcLinearPositioningElement".to_string(),
            super::ifc_schema::IFCLIQUIDTERMINAL => "IfcLiquidTerminal".to_string(),
            super::ifc_schema::IFCMOBILETELECOMMUNICATIONSAPPLIANCE => "IfcMobileTelecommunicationsAppliance".to_string(),
            super::ifc_schema::IFCMOORINGDEVICE => "IfcMooringDevice".to_string(),
            super::ifc_schema::IFCNAVIGATIONELEMENT => "IfcNavigationElement".to_string(),
            super::ifc_schema::IFCPAVEMENT => "IfcPavement".to_string(),
            super::ifc_schema::IFCRAIL => "IfcRail".to_string(),
            super::ifc_schema::IFCREINFORCEDSOIL => "IfcReinforcedSoil".to_string(),
            super::ifc_schema::IFCSIGNAL => "IfcSignal".to_string(),
            super::ifc_schema::IFCTRACKELEMENT => "IfcTrackElement".to_string(),
            super::ifc_schema::IFCALIGNMENT => "IfcAlignment".to_string(),
            super::ifc_schema::IFCBEARING => "IfcBearing".to_string(),
            super::ifc_schema::IFCBOREHOLE => "IfcBorehole".to_string(),
            super::ifc_schema::IFCCAISSONFOUNDATION => "IfcCaissonFoundation".to_string(),
            super::ifc_schema::IFCCONVEYORSEGMENT => "IfcConveyorSegment".to_string(),
            super::ifc_schema::IFCDISTRIBUTIONBOARD => "IfcDistributionBoard".to_string(),
            super::ifc_schema::IFCELECTRICFLOWTREATMENTDEVICE => "IfcElectricFlowTreatmentDevice".to_string(),
            super::ifc_schema::IFCGEOMODEL => "IfcGeomodel".to_string(),
            super::ifc_schema::IFCGEOSLICE => "IfcGeoslice".to_string(),
            super::ifc_schema::IFCABSORBEDDOSEMEASURE => "IfcAbsorbedDoseMeasure".to_string(),
            super::ifc_schema::IFCACCELERATIONMEASURE => "IfcAccelerationMeasure".to_string(),
            super::ifc_schema::IFCAMOUNTOFSUBSTANCEMEASURE => "IfcAmountOfSubstanceMeasure".to_string(),
            super::ifc_schema::IFCANGULARVELOCITYMEASURE => "IfcAngularVelocityMeasure".to_string(),
            super::ifc_schema::IFCAREAMEASURE => "IfcAreaMeasure".to_string(),
            super::ifc_schema::IFCBOOLEAN => "IfcBoolean".to_string(),
            super::ifc_schema::IFCBOXALIGNMENT => "IfcBoxAlignment".to_string(),
            super::ifc_schema::IFCCOMPLEXNUMBER => "IfcComplexNumber".to_string(),
            super::ifc_schema::IFCCOMPOUNDPLANEANGLEMEASURE => "IfcCompoundPlaneAngleMeasure".to_string(),
            super::ifc_schema::IFCCONTEXTDEPENDENTMEASURE => "IfcContextDependentMeasure".to_string(),
            super::ifc_schema::IFCCOUNTMEASURE => "IfcCountMeasure".to_string(),
            super::ifc_schema::IFCCURVATUREMEASURE => "IfcCurvatureMeasure".to_string(),
            super::ifc_schema::IFCDAYINMONTHNUMBER => "IfcDayInMonthNumber".to_string(),
            super::ifc_schema::IFCDAYLIGHTSAVINGHOUR => "IfcDaylightSavingHour".to_string(),
            super::ifc_schema::IFCDESCRIPTIVEMEASURE => "IfcDescriptiveMeasure".to_string(),
            super::ifc_schema::IFCDIMENSIONCOUNT => "IfcDimensionCount".to_string(),
            super::ifc_schema::IFCDOSEEQUIVALENTMEASURE => "IfcDoseEquivalentMeasure".to_string(),
            super::ifc_schema::IFCDYNAMICVISCOSITYMEASURE => "IfcDynamicViscosityMeasure".to_string(),
            super::ifc_schema::IFCELECTRICCAPACITANCEMEASURE => "IfcElectricCapacitanceMeasure".to_string(),
            super::ifc_schema::IFCELECTRICCHARGEMEASURE => "IfcElectricChargeMeasure".to_string(),
            super::ifc_schema::IFCELECTRICCONDUCTANCEMEASURE => "IfcElectricConductanceMeasure".to_string(),
            super::ifc_schema::IFCELECTRICCURRENTMEASURE => "IfcElectricCurrentMeasure".to_string(),
            super::ifc_schema::IFCELECTRICRESISTANCEMEASURE => "IfcElectricResistanceMeasure".to_string(),
            super::ifc_schema::IFCELECTRICVOLTAGEMEASURE => "IfcElectricVoltageMeasure".to_string(),
            super::ifc_schema::IFCENERGYMEASURE => "IfcEnergyMeasure".to_string(),
            super::ifc_schema::IFCFONTSTYLE => "IfcFontStyle".to_string(),
            super::ifc_schema::IFCFONTVARIANT => "IfcFontVariant".to_string(),
            super::ifc_schema::IFCFONTWEIGHT => "IfcFontWeight".to_string(),
            super::ifc_schema::IFCFORCEMEASURE => "IfcForceMeasure".to_string(),
            super::ifc_schema::IFCFREQUENCYMEASURE => "IfcFrequencyMeasure".to_string(),
            super::ifc_schema::IFCGLOBALLYUNIQUEID => "IfcGloballyUniqueId".to_string(),
            super::ifc_schema::IFCHEATFLUXDENSITYMEASURE => "IfcHeatFluxDensityMeasure".to_string(),
            super::ifc_schema::IFCHEATINGVALUEMEASURE => "IfcHeatingValueMeasure".to_string(),
            super::ifc_schema::IFCHOURINDAY => "IfcHourInDay".to_string(),
            super::ifc_schema::IFCIDENTIFIER => "IfcIdentifier".to_string(),
            super::ifc_schema::IFCILLUMINANCEMEASURE => "IfcIlluminanceMeasure".to_string(),
            super::ifc_schema::IFCINDUCTANCEMEASURE => "IfcInductanceMeasure".to_string(),
            super::ifc_schema::IFCINTEGER => "IfcInteger".to_string(),
            super::ifc_schema::IFCINTEGERCOUNTRATEMEASURE => "IfcIntegerCountRateMeasure".to_string(),
            super::ifc_schema::IFCIONCONCENTRATIONMEASURE => "IfcIonConcentrationMeasure".to_string(),
            super::ifc_schema::IFCISOTHERMALMOISTURECAPACITYMEASURE => "IfcIsothermalMoistureCapacityMeasure".to_string(),
            super::ifc_schema::IFCKINEMATICVISCOSITYMEASURE => "IfcKinematicViscosityMeasure".to_string(),
            super::ifc_schema::IFCLABEL => "IfcLabel".to_string(),
            super::ifc_schema::IFCLENGTHMEASURE => "IfcLengthMeasure".to_string(),
            super::ifc_schema::IFCLINEARFORCEMEASURE => "IfcLinearForceMeasure".to_string(),
            super::ifc_schema::IFCLINEARMOMENTMEASURE => "IfcLinearMomentMeasure".to_string(),
            super::ifc_schema::IFCLINEARSTIFFNESSMEASURE => "IfcLinearStiffnessMeasure".to_string(),
            super::ifc_schema::IFCLINEARVELOCITYMEASURE => "IfcLinearVelocityMeasure".to_string(),
            super::ifc_schema::IFCLOGICAL => "IfcLogical".to_string(),
            super::ifc_schema::IFCLUMINOUSFLUXMEASURE => "IfcLuminousFluxMeasure".to_string(),
            super::ifc_schema::IFCLUMINOUSINTENSITYDISTRIBUTIONMEASURE => "IfcLuminousIntensityDistributionMeasure".to_string(),
            super::ifc_schema::IFCLUMINOUSINTENSITYMEASURE => "IfcLuminousIntensityMeasure".to_string(),
            super::ifc_schema::IFCMAGNETICFLUXDENSITYMEASURE => "IfcMagneticFluxDensityMeasure".to_string(),
            super::ifc_schema::IFCMAGNETICFLUXMEASURE => "IfcMagneticFluxMeasure".to_string(),
            super::ifc_schema::IFCMASSDENSITYMEASURE => "IfcMassDensityMeasure".to_string(),
            super::ifc_schema::IFCMASSFLOWRATEMEASURE => "IfcMassFlowRateMeasure".to_string(),
            super::ifc_schema::IFCMASSMEASURE => "IfcMassMeasure".to_string(),
            super::ifc_schema::IFCMASSPERLENGTHMEASURE => "IfcMassPerLengthMeasure".to_string(),
            super::ifc_schema::IFCMINUTEINHOUR => "IfcMinuteInHour".to_string(),
            super::ifc_schema::IFCMODULUSOFELASTICITYMEASURE => "IfcModulusOfElasticityMeasure".to_string(),
            super::ifc_schema::IFCMODULUSOFLINEARSUBGRADEREACTIONMEASURE => "IfcModulusOfLinearSubgradeReactionMeasure".to_string(),
            super::ifc_schema::IFCMODULUSOFROTATIONALSUBGRADEREACTIONMEASURE => "IfcModulusOfRotationalSubgradeReactionMeasure".to_string(),
            super::ifc_schema::IFCMODULUSOFSUBGRADEREACTIONMEASURE => "IfcModulusOfSubgradeReactionMeasure".to_string(),
            super::ifc_schema::IFCMOISTUREDIFFUSIVITYMEASURE => "IfcMoistureDiffusivityMeasure".to_string(),
            super::ifc_schema::IFCMOLECULARWEIGHTMEASURE => "IfcMolecularWeightMeasure".to_string(),
            super::ifc_schema::IFCMOMENTOFINERTIAMEASURE => "IfcMomentOfInertiaMeasure".to_string(),
            super::ifc_schema::IFCMONETARYMEASURE => "IfcMonetaryMeasure".to_string(),
            super::ifc_schema::IFCMONTHINYEARNUMBER => "IfcMonthInYearNumber".to_string(),
            super::ifc_schema::IFCNORMALISEDRATIOMEASURE => "IfcNormalisedRatioMeasure".to_string(),
            super::ifc_schema::IFCNUMERICMEASURE => "IfcNumericMeasure".to_string(),
            super::ifc_schema::IFCPHMEASURE => "IfcPHMeasure".to_string(),
            super::ifc_schema::IFCPARAMETERVALUE => "IfcParameterValue".to_string(),
            super::ifc_schema::IFCPLANARFORCEMEASURE => "IfcPlanarForceMeasure".to_string(),
            super::ifc_schema::IFCPLANEANGLEMEASURE => "IfcPlaneAngleMeasure".to_string(),
            super::ifc_schema::IFCPOSITIVELENGTHMEASURE => "IfcPositiveLengthMeasure".to_string(),
            super::ifc_schema::IFCPOSITIVEPLANEANGLEMEASURE => "IfcPositivePlaneAngleMeasure".to_string(),
            super::ifc_schema::IFCPOSITIVERATIOMEASURE => "IfcPositiveRatioMeasure".to_string(),
            super::ifc_schema::IFCPOWERMEASURE => "IfcPowerMeasure".to_string(),
            super::ifc_schema::IFCPRESENTABLETEXT => "IfcPresentableText".to_string(),
            super::ifc_schema::IFCPRESSUREMEASURE => "IfcPressureMeasure".to_string(),
            super::ifc_schema::IFCRADIOACTIVITYMEASURE => "IfcRadioActivityMeasure".to_string(),
            super::ifc_schema::IFCRATIOMEASURE => "IfcRatioMeasure".to_string(),
            super::ifc_schema::IFCREAL => "IfcReal".to_string(),
            super::ifc_schema::IFCROTATIONALFREQUENCYMEASURE => "IfcRotationalFrequencyMeasure".to_string(),
            super::ifc_schema::IFCROTATIONALMASSMEASURE => "IfcRotationalMassMeasure".to_string(),
            super::ifc_schema::IFCROTATIONALSTIFFNESSMEASURE => "IfcRotationalStiffnessMeasure".to_string(),
            super::ifc_schema::IFCSECONDINMINUTE => "IfcSecondInMinute".to_string(),
            super::ifc_schema::IFCSECTIONMODULUSMEASURE => "IfcSectionModulusMeasure".to_string(),
            super::ifc_schema::IFCSECTIONALAREAINTEGRALMEASURE => "IfcSectionalAreaIntegralMeasure".to_string(),
            super::ifc_schema::IFCSHEARMODULUSMEASURE => "IfcShearModulusMeasure".to_string(),
            super::ifc_schema::IFCSOLIDANGLEMEASURE => "IfcSolidAngleMeasure".to_string(),
            super::ifc_schema::IFCSOUNDPOWERMEASURE => "IfcSoundPowerMeasure".to_string(),
            super::ifc_schema::IFCSOUNDPRESSUREMEASURE => "IfcSoundPressureMeasure".to_string(),
            super::ifc_schema::IFCSPECIFICHEATCAPACITYMEASURE => "IfcSpecificHeatCapacityMeasure".to_string(),
            super::ifc_schema::IFCSPECULAREXPONENT => "IfcSpecularExponent".to_string(),
            super::ifc_schema::IFCSPECULARROUGHNESS => "IfcSpecularRoughness".to_string(),
            super::ifc_schema::IFCTEMPERATUREGRADIENTMEASURE => "IfcTemperatureGradientMeasure".to_string(),
            super::ifc_schema::IFCTEXT => "IfcText".to_string(),
            super::ifc_schema::IFCTEXTALIGNMENT => "IfcTextAlignment".to_string(),
            super::ifc_schema::IFCTEXTDECORATION => "IfcTextDecoration".to_string(),
            super::ifc_schema::IFCTEXTFONTNAME => "IfcTextFontName".to_string(),
            super::ifc_schema::IFCTEXTTRANSFORMATION => "IfcTextTransformation".to_string(),
            super::ifc_schema::IFCTHERMALADMITTANCEMEASURE => "IfcThermalAdmittanceMeasure".to_string(),
            super::ifc_schema::IFCTHERMALCONDUCTIVITYMEASURE => "IfcThermalConductivityMeasure".to_string(),
            super::ifc_schema::IFCTHERMALEXPANSIONCOEFFICIENTMEASURE => "IfcThermalExpansionCoefficientMeasure".to_string(),
            super::ifc_schema::IFCTHERMALRESISTANCEMEASURE => "IfcThermalResistanceMeasure".to_string(),
            super::ifc_schema::IFCTHERMALTRANSMITTANCEMEASURE => "IfcThermalTransmittanceMeasure".to_string(),
            super::ifc_schema::IFCTHERMODYNAMICTEMPERATUREMEASURE => "IfcThermodynamicTemperatureMeasure".to_string(),
            super::ifc_schema::IFCTIMEMEASURE => "IfcTimeMeasure".to_string(),
            super::ifc_schema::IFCTIMESTAMP => "IfcTimeStamp".to_string(),
            super::ifc_schema::IFCTORQUEMEASURE => "IfcTorqueMeasure".to_string(),
            super::ifc_schema::IFCVAPORPERMEABILITYMEASURE => "IfcVaporPermeabilityMeasure".to_string(),
            super::ifc_schema::IFCVOLUMEMEASURE => "IfcVolumeMeasure".to_string(),
            super::ifc_schema::IFCVOLUMETRICFLOWRATEMEASURE => "IfcVolumetricFlowRateMeasure".to_string(),
            super::ifc_schema::IFCWARPINGCONSTANTMEASURE => "IfcWarpingConstantMeasure".to_string(),
            super::ifc_schema::IFCWARPINGMOMENTMEASURE => "IfcWarpingMomentMeasure".to_string(),
            super::ifc_schema::IFCYEARNUMBER => "IfcYearNumber".to_string(),
            super::ifc_schema::IFCARCINDEX => "IfcArcIndex".to_string(),
            super::ifc_schema::IFCAREADENSITYMEASURE => "IfcAreaDensityMeasure".to_string(),
            super::ifc_schema::IFCBINARY => "IfcBinary".to_string(),
            super::ifc_schema::IFCCARDINALPOINTREFERENCE => "IfcCardinalPointReference".to_string(),
            super::ifc_schema::IFCDATE => "IfcDate".to_string(),
            super::ifc_schema::IFCDATETIME => "IfcDateTime".to_string(),
            super::ifc_schema::IFCDAYINWEEKNUMBER => "IfcDayInWeekNumber".to_string(),
            super::ifc_schema::IFCDURATION => "IfcDuration".to_string(),
            super::ifc_schema::IFCLANGUAGEID => "IfcLanguageId".to_string(),
            super::ifc_schema::IFCLINEINDEX => "IfcLineIndex".to_string(),
            super::ifc_schema::IFCNONNEGATIVELENGTHMEASURE => "IfcNonNegativeLengthMeasure".to_string(),
            super::ifc_schema::IFCPOSITIVEINTEGER => "IfcPositiveInteger".to_string(),
            super::ifc_schema::IFCPROPERTYSETDEFINITIONSET => "IfcPropertySetDefinitionSet".to_string(),
            super::ifc_schema::IFCSOUNDPOWERLEVELMEASURE => "IfcSoundPowerLevelMeasure".to_string(),
            super::ifc_schema::IFCSOUNDPRESSURELEVELMEASURE => "IfcSoundPressureLevelMeasure".to_string(),
            super::ifc_schema::IFCTEMPERATURERATEOFCHANGEMEASURE => "IfcTemperatureRateOfChangeMeasure".to_string(),
            super::ifc_schema::IFCTIME => "IfcTime".to_string(),
            super::ifc_schema::IFCURIREFERENCE => "IfcURIReference".to_string(),
            super::ifc_schema::IFCSTRIPPEDOPTIONAL => "IfcStrippedOptional".to_string(),
            super::ifc_schema::IFCWELLKNOWNTEXTLITERAL => "IfcWellKnownTextLiteral".to_string(),
            _ => String::new(),
        }
    }

    pub fn is_ifc_element(&self, type_code: u32) -> bool {
        self.ifc_elements.contains(&type_code)
    }

    pub fn get_ifc_element_list(&self) -> &HashSet<u32> {
        &self.ifc_elements
    }
}

impl Default for IfcSchemaManager {
    fn default() -> Self {
        Self::new()
    }
}
