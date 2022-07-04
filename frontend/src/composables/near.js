import { ref, onMounted } from "vue";
import {
  wallet,
  CONTRACT_ID,
  getProperty,
  getProperties,
  getOwnProperty,
  addProperty,
  buyProperty,
  deleteProperty,
  putPropertyOffSale,
  putPropertyOnSale,
} from '@/services/near';

export const useProperties = () => {
  const properties = ref([]);
  const err = ref(null);

  onMounted(async () => {
    try {
      properties.value = await getProperties(0);
    } catch (e) {
      err.value = e;
      console.log(err.value);
    }
  });
  return {
    properties,
    CONTRACT_ID,
    getProperty,
    getProperties,
    getOwnProperty,
    addProperty,
    buyProperty,
    deleteProperty,
    putPropertyOffSale,
    putPropertyOnSale,
  }
}

export const useWallet = () => {
  const accountId = ref('')
  const err = ref(null)

  onMounted(async () => {
    try {
      accountId.value = wallet.getAccountId()
    } catch (e) {
      err.value = e;
      console.error(err.value);
    }
  });

  const handleSignIn = () => {
    wallet.requestSignIn({
      contractId: CONTRACT_ID,
      methodNames: [] // add methods names to restrict access
    })
  };

  const handleSignOut = () => {
    wallet.signOut()
    accountId.value = ''
  };

  return {
    accountId,
    signIn: handleSignIn,
    signOut: handleSignOut
  }
}
