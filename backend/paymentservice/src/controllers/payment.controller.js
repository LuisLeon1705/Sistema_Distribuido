const Payment = require("../models/payment");
const axios = require("axios");

const serializePayment = (p) => {
  const payment = p && p.toJSON ? p.toJSON() : (p || {});
  return {
    id: payment.id,
    code: payment.code || null,
    user_id: payment.userId || payment.user_id || null,
    order_id: payment.orderId || payment.order_id || null,
    amount: payment.amount,
    currency: payment.currency,
    status: payment.status ? String(payment.status).toLowerCase() : null,
    transaction_id: payment.transactionId || payment.transaction_id || payment.reference || null,
    method: payment.method || null,
    reference: payment.reference || null,
    metadata: payment.metadata || null,
    created_at: payment.createdAt || payment.created_at || null,
    updated_at: payment.updatedAt || payment.updated_at || null,
  };
};

exports.getPayments = async (req, res) => {
  try {
    const whereClause = {};
    if (req.query.orderId || req.query.order_id) whereClause.orderId = req.query.orderId || req.query.order_id;
    if (req.query.userId || req.query.user_id) whereClause.userId = req.query.userId || req.query.user_id;
    const payments = await Payment.findAll({ where: whereClause });
    res.json(payments.map(serializePayment));
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};

exports.getPaymentById = async (req, res) => {
  try {
    const payment = await Payment.findByPk(req.params.id);
    if (!payment) return res.status(404).json({ message: "Payment not found" });
    res.json(serializePayment(payment));
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};

exports.processPayment = async (req, res) => {
  try {
    const body = req.body || {};
    const authToken = req.headers["authorization"];

    const orderId = body.orderId || body.order_id;
    const userId = body.userId || body.user_id || null;
    const amount = body.amount || body.total || 0;
    const currency = body.currency || body.curr || 'USD';
    const method = body.method || body.payment_method || null;
    const transactionId = body.transactionId || body.transaction_id || body.reference || null;
    const metadata = body.metadata || body.meta || null;

    const payment = await Payment.create({
      orderId,
      userId,
      amount,
      currency,
      status: "PENDING",
      method,
      reference: transactionId,
      metadata,
    });

    try {
      const code = `PAY-${Date.now().toString(36).toUpperCase()}-${Math.random().toString(36).slice(2,8).toUpperCase()}`;
      await payment.update({ code });
      payment.code = code;
    } catch (err) {
      console.error('Failed to set payment code:', err.message || err);
    }

    try {
      await axios.put(
        `http://ordersservice:8080/api/orders/${orderId}/status`,
        { status: "PENDING" },
        { headers: { Authorization: authToken } }
      );
      console.log(`Order ${orderId} status updated to PENDING`);
    } catch (error) {
      console.error(`Failed to update order ${orderId} status to PENDING:`, error.message);
    }

    setTimeout(async () => {
      const isSuccess = Math.random() > 0.2;

      const finalStatus = isSuccess ? "COMPLETED" : "FAILED";
      const mockTransaction = `tx_${Math.floor(Math.random() * 1000000)}`;

      await payment.update({
        status: finalStatus,
        transactionId: mockTransaction,
        reference: mockTransaction,
        code: payment.code || `PAY-${Date.now().toString(36).toUpperCase()}-${Math.random().toString(36).slice(2,8).toUpperCase()}`
      });

      console.log(`Payment processed for Order ${orderId}: ${finalStatus}`);

      try {
        await axios.put(
          `http://ordersservice:8080/api/orders/${orderId}/status`,
          { status: finalStatus },
          { headers: { Authorization: authToken } }
        );
        console.log(`Order ${orderId} status updated to ${finalStatus}`);
      } catch (error) {
        console.error(`Failed to update order ${orderId} status to ${finalStatus}:`, error.message);
      }

      if (String(finalStatus).toUpperCase() === "FAILED") {
        try {
          console.log(`Payment failed for Order ${orderId}, preparing rejection notification...`);

          const userResponse = await axios.get(
            'http://authservice:8000/api/users/me',
            { headers: { Authorization: authToken } }
          );
          const userData = userResponse.data;

          let orderCode = null;
          try {
            const orderResp = await axios.get(`http://ordersservice:8080/api/orders/${orderId}`, { headers: { Authorization: authToken } });
            if (orderResp && orderResp.data) {
              orderCode = orderResp.data.code || orderResp.data.order_code || null;
            }
          } catch (ordErr) {
            console.debug(`  Could not fetch order ${orderId} for code:`, ordErr.message || ordErr);
          }

          const payload = {
            order_id: orderId,
            customer_email: userData.email,
            customer_name: userData.username,
            amount: parseFloat(amount),
            transaction_id: mockTransaction,
            rejection_reason: "Fondos insuficientes o datos de tarjeta incorrectos"
          };
          if (orderCode) payload.order_code = orderCode;

          console.log('Sending payment rejection notification payload:', payload);

          await axios.post('http://notificationsservice:8000/notifications/payment-rejected', payload);
          console.log(`Payment rejection notification sent for Order ${orderId}`);
        } catch (notifError) {
          console.error(`Failed to send payment rejection notification for Order ${orderId}:`, notifError.message || notifError);
        }
      }

    }, 5000);

    res.status(201).json(serializePayment(payment));

  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};
