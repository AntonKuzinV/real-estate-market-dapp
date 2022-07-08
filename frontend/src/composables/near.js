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
import router from '@/router';

export const useProperties = () => {
  const properties = ref([]);
  const err = ref(null);

  onMounted(async () => {
    try {
      await updateProperties()
    } catch (e) {
      err.value = e;
      console.log(err.value);
    }
  });
  const updateProperties = async () => {
    properties.value = await getProperties(0);
  }
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
    updateProperties,
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
    accountId.value = wallet.getAccountId()
    router.push('/');
  };

  return {
    accountId,
    signIn: handleSignIn,
    signOut: handleSignOut
  }
}
