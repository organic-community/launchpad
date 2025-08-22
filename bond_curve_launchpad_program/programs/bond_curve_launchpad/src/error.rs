// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.
//
// Docs: https://docs.codigo.ai/c%C3%B3digo-interface-description-language/specification#errors

use anchor_lang::prelude::*;

#[error_code]
pub enum BondCurveLaunchpadError {
	#[msg("Insufficient funds for transaction")]
	InsufficientFunds,
	#[msg("Invalid bond curve type")]
	InvalidCurveType,
	#[msg("Unauthorized access")]
	UnauthorizedAccess,
	#[msg("Token not eligible for graduation")]
	TokenNotEligibleForGraduation,
	#[msg("Bundling detected, transfer not allowed")]
	BundlingDetected,
	#[msg("Invalid amount")]
	InvalidAmount,
}
