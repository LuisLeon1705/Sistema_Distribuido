const { DataTypes } = require("sequelize");
const sequelize = require("../config/database");

const Payment = sequelize.define("Payment", {
  id: {
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4,
    primaryKey: true,
  },
  orderId: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  userId: {
    type: DataTypes.UUID,
    allowNull: true,
  },
  amount: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: false,
  },
  currency: {
    type: DataTypes.STRING,
    defaultValue: "USD",
  },
  status: {
    type: DataTypes.ENUM("PENDING", "COMPLETED", "FAILED", "DECLINED"),
    defaultValue: "PENDING",
  },
  transactionId: {
    type: DataTypes.STRING,
  },
  code: {
    type: DataTypes.STRING,
    unique: true,
  }
  ,method: {
    type: DataTypes.STRING,
  },
  reference: {
    type: DataTypes.STRING,
  },
  metadata: {
    type: DataTypes.JSONB,
    allowNull: true,
  }
});

module.exports = Payment;