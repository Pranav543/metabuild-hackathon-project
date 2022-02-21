import { utils } from "near-api-js";
const BN = require("bn.js");

// Query investments for a single investor
export const getInvestments = async (address) => {
  const allInvestorInvestments = {
    investments: [],
  };
  const investments = await window.contract.get_investor_investments({
    investor: address,
  });

  if (investments.length > 0) {
    investments.forEach((inv) => {
      allInvestorInvestments["investments"] = allInvestorInvestments[
        "investments"
      ].concat(inv["investments"]);
    });
  }

  return allInvestorInvestments;
};

// Querying current ELE Balance for address
export const getElementBalance = async (address, elementTokenContract) => {
  const balance = await elementTokenContract.ft_balance_of({
    account_id: address,
  });

  return balance;
};

/**
 * Invest
 */

// Invest ELE to a hex
export const investElement = async (
  elementTokenContract,
  hex,
  hexIndex,
  amount
) => {
  const result = await elementTokenContract.ft_transfer_call(
    {
      msg: `invest::${hex}::${hexIndex}`,
      amount: amount,
      receiver_id: window.contract.contractId,
    },
    300000000000000, // attached GAS (optional)
    new BN(utils.format.parseNearAmount("0.000000000000000000000001"))
  );
  return result;
};

// Withdraw all
export const withdrawElement = async (address) => {
  const result = await window.contract.withdraw(
    {
      investor: address,
    },
    300000000000000, // attached GAS (optional)
    new BN(utils.format.parseNearAmount("0.000000000000000000000001"))
  );
  return result;
};
