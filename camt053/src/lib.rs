use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Document {
    #[serde(rename = "BkToCstmrStmt")]
    pub bank_to_customer: BankToCustomerStatement,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BankToCustomerStatement {
    #[serde(rename = "Stmt")]
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Statement {
    #[serde(rename = "Bal")]
    pub balance: Vec<Balance>,
    #[serde(rename = "Acct")]
    pub account: Account,
    #[serde(rename = "Ntry")]
    pub entries: Vec<Entry>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Balance {
    #[serde(rename = "Tp")]
    pub balance_type: BalanceType,
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub credit_or_debit: CreditDebitIndicator,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    pub credit_or_property: CodeOrProperty,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CodeOrProperty {
    #[serde(rename = "Cd")]
    pub code: BalanceCodeValue,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BalanceCodeValue {
    #[serde(rename = "$value")]
    pub value: BalanceCode,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum BalanceCode {
    #[serde(rename = "PRCD")]
    PrevClosedBooked,
    #[serde(rename = "OPBD")]
    OpeningBooked,
    #[serde(rename = "CLBD")]
    ClosingBooked,
    #[serde(rename = "CLAV")]
    ClosingAvailable,
    #[serde(rename = "FWAV")]
    ForwardAvailable,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CreditDebitIndicator {
    #[serde(rename = "$value")]
    pub value: CreditOrDebit,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Copy)]
pub enum CreditOrDebit {
    #[serde(rename = "CRDT")]
    Credit,
    #[serde(rename = "DBIT")]
    Debit,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Entry {
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub credit_or_debit: CreditDebitIndicator,
    #[serde(rename = "BookgDt")]
    pub booking_date: Date,
    #[serde(rename = "ValDt")]
    pub value_date: Date,
    #[serde(rename = "BkTxCd")]
    pub bank_transaction_code: BankTransactionCode,
    #[serde(rename = "Chrgs")]
    pub charges: Option<Charges>,
    #[serde(rename = "NtryDtls")]
    pub details: Option<EntryDetails>,
    #[serde(rename = "AddtlNtryInf")]
    pub additional_info: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BankTransactionCode {
    #[serde(rename = "Domn")]
    pub domain: Option<Domain>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Domain {
    #[serde(rename = "Cd")]
    pub code: DomainCodeValue,
    #[serde(rename = "Fmly")]
    pub family: DomainFamily,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DomainFamily {
    #[serde(rename = "Cd")]
    pub code: DomainFamilyCodeValue,
    #[serde(rename = "SubFmlyCd")]
    pub sub_family_code: DomainSubFamilyCodeValue,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DomainCodeValue {
    #[serde(rename = "$value")]
    pub value: DomainCode,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum DomainCode {
    #[serde(rename = "PMNT")]
    Payment,
    #[serde(rename = "XTND")]
    Extended,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DomainFamilyCodeValue {
    #[serde(rename = "$value")]
    pub value: DomainFamilyCode,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DomainSubFamilyCodeValue {
    #[serde(rename = "$value")]
    pub value: DomainSubFamilyCode,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum DomainFamilyCode {
    #[serde(rename = "CCRD")]
    CardPayment,
    #[serde(rename = "ICDT")]
    IssuedCreditTransfers,
    #[serde(rename = "RCDT")]
    ReceivedCreditTransfers,
    #[serde(rename = "RDDT")]
    ReceivedDirectDebits,
    #[serde(rename = "NTAV")]
    NotAvailable,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum DomainSubFamilyCode {
    #[serde(rename = "POSD")]
    PointOfSaleDebit,
    #[serde(rename = "AUTT")]
    AutomaticTransfer,
    #[serde(rename = "PMDD")]
    PaymentDirectDebit,
    #[serde(rename = "ESDD")]
    SepaCoreDirectDebit,
    #[serde(rename = "SALA")]
    Salary,
    #[serde(rename = "ESCT")]
    CreditTransfer,
    #[serde(rename = "STDO")]
    StandingOrder,
    #[serde(rename = "NTAV")]
    NotAvailable,
    #[serde(rename = "OTHR")]
    Other,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EntryDetails {
    #[serde(rename = "Btch", default)]
    pub batch: Batch,
    #[serde(rename = "TxDtls", default)]
    pub transactions: Vec<TransactionDetails>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
pub struct Batch {
    #[serde(rename = "NbOfTxs")]
    pub number_of_transactions: usize,
    // Redundant fields.
    // #[serde(rename = "TtlAmt")]
    // pub total_amount: Amount,
    // #[serde(rename = "CdtDbtInd")]
    // pub credit_or_debit: CreditDebitIndicator,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TransactionDetails {
    #[serde(rename = "Refs")]
    pub refs: References,
    #[serde(rename = "Amt")]
    pub amount: Option<Amount>,
    #[serde(rename = "CdtDbtInd")]
    pub credit_or_debit: Option<CreditDebitIndicator>,
    #[serde(rename = "AmtDtls")]
    pub amount_details: Option<AmountDetails>,
    #[serde(rename = "Chrgs")]
    pub charges: Option<Charges>,
    #[serde(rename = "RltdPties")]
    pub related_parties: Option<RelatedParties>,
    #[serde(rename = "RmtInf")]
    pub remittance_info: Option<RemittanceInfo>,
    #[serde(rename = "AddtlTxInf")]
    pub additional_info: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RemittanceInfo {
    #[serde(rename = "Ustrd")]
    pub unstructured: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct References {
    #[serde(rename = "AcctSvcrRef")]
    pub account_servicer_reference: Option<String>,
    // // may be Some("NOTPROVIDED")
    // #[serde(rename = "EndToEndId")]
    // pub end_to_end_id: String,
    // // may be Some("NOTPROVIDED") or Some("000000000")
    // #[serde(rename = "InstrId")]
    // pub instruction_id: Option<String>,
    // #[serde(rename = "TxId")]
    // pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RelatedParties {
    #[serde(rename = "Dbtr")]
    pub debtor: Option<Party>,
    #[serde(rename = "Cdtr")]
    pub creditor: Option<Party>,
    #[serde(rename = "CdtrAcct")]
    pub creditor_account: Option<Account>,
    #[serde(rename = "DbtrAcct")]
    pub debtor_account: Option<Account>,
    #[serde(rename = "UltmtDbtr")]
    pub ultimate_debtor: Option<Party>,
    #[serde(rename = "UltmtCdtr")]
    pub ultimate_creditor: Option<Party>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Party {
    #[serde(rename = "Nm")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Account {
    #[serde(rename = "Id")]
    pub id: AccountIdWrapper,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AccountIdWrapper {
    #[serde(rename = "$value")]
    pub value: AccountId,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum AccountId {
    #[serde(rename = "IBAN")]
    Iban(String),
    #[serde(rename = "Othr")]
    Other(OtherAccountId),
}

impl AccountId {
    pub fn as_str_id(&self) -> &str {
        match self {
            AccountId::Iban(value) => value.as_str(),
            AccountId::Other(OtherAccountId { id }) => id.as_str(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct OtherAccountId {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Amount {
    #[serde(rename = "Ccy")]
    pub currency: String,
    #[serde(rename = "$value")]
    pub value: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AmountWithExchange {
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CcyXchg")]
    pub currency_exchange: Option<CurrencyExchange>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CurrencyExchange {
    #[serde(rename = "SrcCcy")]
    pub source_currency: String,
    #[serde(rename = "TrgtCcy")]
    pub target_currency: String,
    #[serde(rename = "XchgRate")]
    pub exchange_rate: ExchangeRate,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ExchangeRate {
    #[serde(rename = "$value")]
    pub value: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AmountDetails {
    // Actual passed amount.
    #[serde(rename = "InstdAmt")]
    pub instructed: Option<AmountWithExchange>,
    // Specified transaction amount, before charge deduction.
    #[serde(rename = "TxAmt")]
    pub transaction: AmountWithExchange,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Charges {
    #[serde(rename = "TtlChrgsAndTaxAmt")]
    pub total: Option<Amount>,
    #[serde(rename = "Rcrd")]
    pub records: Vec<ChargeRecord>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ChargeRecord {
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub credit_or_debit: CreditDebitIndicator,
    #[serde(rename = "ChrgInclInd", default)]
    pub is_charge_included: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Date {
    #[serde(rename = "Dt")]
    pub date: chrono::NaiveDate,
}
