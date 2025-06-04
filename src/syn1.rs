//! Модуль `syn1`
//!
//! Загрузка бинарного формата SYN1 (ASG):
//! - Little Endian, VarInt
//! - Поддержка всех узлов/рёбер, включая payload
//!
//! Ошибки обрабатываются через SynapseError::Serialization.

use std::fs::File;
use std::io::Read;
use byteorder::ReadBytesExt;

use crate::asg::{ASG, Edge, Node, NodeID};
use crate::nodecodes::{EdgeType, NodeType};
use crate::{SynapseError, SynapseResult};

/// Загрузка ASG из бинарного файла формата SYN1.
pub fn load_syn1(path: &str) -> SynapseResult<ASG> {
    let mut file = File::open(path)
        .map_err(|e| SynapseError::Serialization(format!("Failed to open file: {}", e)))?;

    let mut header = [0u8; 4];
    file.read_exact(&mut header)
        .map_err(|e| SynapseError::Serialization(format!("Failed to read header: {}", e)))?;
    if &header != b"SYN1" {
        return Err(SynapseError::Serialization("Invalid SYN1 header".into()));
    }

    let _version = file
        .read_u8()
        .map_err(|e| SynapseError::Serialization(format!("Failed to read version: {}", e)))?;

    let node_count = read_varint(&mut file)?;

    let mut nodes = Vec::new();
    for _ in 0..node_count {
        let node_type_code = read_varint(&mut file)?;
        let node_type = NodeType::try_from(node_type_code as u32)
            .map_err(|_| SynapseError::Serialization(format!("Unknown NodeType: {}", node_type_code)))?;

        let node_id = read_varint(&mut file)? as NodeID;

        let payload_length = read_varint(&mut file)? as usize;
        let mut payload = None;
        if payload_length > 0 {
            let mut buf = vec![0u8; payload_length];
            file.read_exact(&mut buf)
                .map_err(|e| SynapseError::Serialization(format!("Failed to read node payload: {}", e)))?;
            payload = Some(buf);
        }

        let edge_count = read_varint(&mut file)? as usize;
        let mut edges = Vec::new();
        for _ in 0..edge_count {
            let edge_type_code = read_varint(&mut file)?;
            let edge_type = EdgeType::try_from(edge_type_code as u32)
                .map_err(|_| SynapseError::Serialization(format!("Unknown EdgeType: {}", edge_type_code)))?;

            let target_node_id = read_varint(&mut file)? as NodeID;

            let edge_payload_length = read_varint(&mut file)? as usize;
            let mut edge_payload = None;
            if edge_payload_length > 0 {
                let mut buf = vec![0u8; edge_payload_length];
                file.read_exact(&mut buf)
                    .map_err(|e| SynapseError::Serialization(format!("Failed to read edge payload: {}", e)))?;
                edge_payload = Some(buf);
            }

            edges.push(Edge::new(edge_type, target_node_id, edge_payload));
        }

        let mut node = Node::new(node_id, node_type, payload);
        node.edges = edges;
        nodes.push(node);
    }

    Ok(ASG { nodes })
}

fn read_varint<R: Read>(reader: &mut R) -> SynapseResult<u64> {
    let mut result = 0u64;
    let mut shift = 0u8;

    loop {
        let byte = reader
            .read_u8()
            .map_err(|e| SynapseError::Serialization(format!("Failed to read varint: {}", e)))?;

        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err(SynapseError::Serialization("VarInt too long".into()));
        }
    }

    Ok(result)
}

impl TryFrom<u32> for NodeType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use NodeType::*;
        let node_types = [
            LiteralInt, LiteralFloat, LiteralBool, LiteralString, LiteralUnit,
            BinaryOperation, UnaryOperation, Conditional, RecordFieldAccess, Dereference,
            VariableDefinition, VariableReference, Lambda, Application,
            TypeInt, TypeFloat, TypeBool, TypeString, TypeUnit, TypeFunction,
            TypeVariable, ForAll, TypeRecord, FieldDefinition, TypeADT, ADTVariant,
            TypeLinear, TypeSharedRef, TypeMutableRef, TypeLifetime, TypeResult,
            TypeErrorUnion, TypeTrait, TraitMethodDecl, ForeignTypeDecl,
            EffectIO, EffectConsole, EffectFSRead, EffectFSWrite, EffectNetwork,
            EffectState, EffectRandom, EffectExcep, EffectNonTerm, EffectPure,
            DataRecordInit, DataADTInit, DataOk, DataErr,
            PerformEffect, MatchResult, MatchADT, MacroDefinition, MacroInvocation,
            ModuleRoot, ImportDeclaration, ExportDeclaration, ImportAlias,
            ForeignFunctionDecl, ForeignBlock,
            Proof, Specification, Assume, Assert,
            TestCase, TestSuite, Assertion, PropertyDefinition, InputGenerator,
            MatchCase, ImplMethodDef, TraitImpl, Concurrency
        ];

        node_types.get(value as usize).cloned().ok_or(())
    }
}

impl TryFrom<u32> for EdgeType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use EdgeType::*;
        let edge_types = [
            DataInput, ControlFlowNext, Condition, ThenBranch, ElseBranch,
            ScopeLink, LambdaParameter, LambdaBody, DefinitionLink, BindsVariable,
            ApplicationFunction, ApplicationArgument,
            TypeAnnotation, FunctionParamType, FunctionReturnType,
            TypeVarBinding, TypeBody, Constraint, LinearInnerType,
            RefInnerType, RefLifetime, LifetimeBound, ResultOkType, ResultErrType,
            RecordField, FieldName, FieldType, FieldValue, FieldTarget,
            HasVariant, VariantName, VariantParam, VariantArgValue, VariantTarget,
            ProducesEffect, MatchInput, MatchOkBranch, MatchErrBranch,
            MatchBranch, MatchesVariant, CaseBody,
            ImportsFromModule, ImportsSymbol, ImportsAll, ExportsSymbol, ModuleContains,
            HasFFISignature, UsesABI, LinksToLibrary,
            SpecifiesCode, ProofStepDependsOn, ReliesOnAssumption,
            TestsFunction, ProvidesInput, MakesAssertion, ChecksProperty, InputForProperty,
            MacroBody, MacroInputAST, InvokesMacro,
            ImplementsTrait, ForType, ProvidesImpl, ImplementsMethod,
            RootExpression
        ];

        edge_types.get(value as usize).cloned().ok_or(())
    }
}
