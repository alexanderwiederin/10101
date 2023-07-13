use crate::db::payments;
use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use lightning::chain::keysinterface::SpendableOutputDescriptor;
use lightning::chain::transaction::OutPoint;
use lightning::ln::PaymentHash;
use lightning::ln::PaymentPreimage;
use lightning::ln::PaymentSecret;
use ln_dlc_node::node;
use ln_dlc_node::HTLCStatus;
use ln_dlc_node::MillisatAmount;
use ln_dlc_node::PaymentFlow;
use ln_dlc_node::PaymentInfo;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct NodeStorage {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl NodeStorage {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl node::Storage for NodeStorage {
    // Payments

    fn insert_payment(&self, payment_hash: PaymentHash, info: PaymentInfo) -> Result<()> {
        let mut conn = self.pool.get()?;
        payments::insert((payment_hash, info), &mut conn)
    }

    fn merge_payment(
        &self,
        payment_hash: &PaymentHash,
        flow: PaymentFlow,
        amt_msat: MillisatAmount,
        htlc_status: HTLCStatus,
        preimage: Option<PaymentPreimage>,
        secret: Option<PaymentSecret>,
    ) -> Result<()> {
        let mut conn = self.pool.get()?;

        match payments::get(*payment_hash, &mut conn)? {
            Some(_) => {
                payments::update(
                    *payment_hash,
                    htlc_status,
                    amt_msat,
                    preimage,
                    secret,
                    &mut conn,
                )?;
            }
            None => {
                payments::insert(
                    (
                        *payment_hash,
                        PaymentInfo {
                            preimage,
                            secret,
                            status: htlc_status,
                            amt_msat,
                            flow,
                            timestamp: OffsetDateTime::now_utc(),
                            description: "".to_string(),
                        },
                    ),
                    &mut conn,
                )?;
            }
        }

        Ok(())
    }

    fn get_payment(
        &self,
        payment_hash: &PaymentHash,
    ) -> Result<Option<(PaymentHash, PaymentInfo)>> {
        let mut conn = self.pool.get()?;
        payments::get(*payment_hash, &mut conn)
    }

    fn all_payments(&self) -> Result<Vec<(PaymentHash, PaymentInfo)>> {
        let mut conn = self.pool.get()?;
        payments::get_all(&mut conn)
    }

    // Spendable outputs

    fn insert_spendable_output(&self, output: SpendableOutputDescriptor) -> Result<()> {
        let mut conn = self.pool.get()?;
        crate::db::spendable_outputs::insert(&mut conn, output)?;

        Ok(())
    }

    fn get_spendable_output(
        &self,
        outpoint: &OutPoint,
    ) -> Result<Option<SpendableOutputDescriptor>> {
        let mut conn = self.pool.get()?;
        crate::db::spendable_outputs::get(&mut conn, outpoint)
    }

    fn all_spendable_outputs(&self) -> Result<Vec<SpendableOutputDescriptor>> {
        let mut conn = self.pool.get()?;
        crate::db::spendable_outputs::get_all(&mut conn)
    }
}