import { keyStores, Near, utils, WalletConnection } from 'near-api-js';
import BN from 'bn.js';

export const CONTRACT_ID = process.env.VUE_APP_CONTRACT_ID;
const gas = new BN(process.env.VUE_APP_gas);

export const near = new Near({
  networkId: process.env.VUE_APP_networkId,
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: process.env.VUE_APP_nodeUrl,
  walletUrl: process.env.VUE_APP_walletUrl,
});

export const wallet = new WalletConnection(near, 'real-estate-market');

// --------------------------------------------------------------------------
// functions to call contract Public VIEW methods
// --------------------------------------------------------------------------

export const getProperties = (from_index) => {
  return wallet.account().viewFunction(CONTRACT_ID, 'get_properties', { from_index });
};

export const getProperty = (propertyId) => {
  return wallet.account().viewFunction(CONTRACT_ID, 'get_property', { property_id: propertyId });
};

export const getOwnProperty = () => {
  return wallet.account().viewFunction(CONTRACT_ID, 'get_own_property');
};

// --------------------------------------------------------------------------
// functions to call contract Public CHANGE methods
// --------------------------------------------------------------------------

export const addProperty = (propertyName, propertyType, location, rooms, floor, storeys, squarespace, price) => {
  return wallet.account().functionCall({
    contractId: CONTRACT_ID,
    methodName: 'add_property',
    gas,
    args: {
      property_name: propertyName,
      property_type: propertyType,
      location: location,
      rooms: rooms,
      floor: floor,
      storeys: storeys,
      squarespace: squarespace,
      price: price,
    },
  });
};

export const buyProperty = (propertyId, price) => {
  return wallet.account().functionCall({
    contractId: CONTRACT_ID,
    methodName: 'buy_property',
    gas,
    args: { property_id: propertyId },
    attachedDeposit: utils.format.parseNearAmount(price),
  });
};

export const deleteProperty = (propertyId) => {
  return wallet.account().functionCall({
    contractId: CONTRACT_ID,
    methodName: 'delete_property',
    gas,
    args: { property_id: propertyId },
  });
};

export const putPropertyOnSale = (propertyId) => {
  return wallet.account().functionCall({
    contractId: CONTRACT_ID,
    methodName: 'put_property_on_sale',
    gas,
    args: { property_id: propertyId },
  });
};

export const putPropertyOffSale = (propertyId) => {
  return wallet.account().functionCall({
    contractId: CONTRACT_ID,
    methodName: 'put_property_off_sale',
    gas,
    args: { property_id: propertyId },
  });
};
