<template>
  <TheHeader
      :accountId="accountId"
      :signIn="signIn"
      :signOut="signOut"
  />
  <TheProperties
      :properties="properties"
      :contract-id="CONTRACT_ID"
      :buy-property="buyProperty"
      :delete-property="deleteProperty"
      :put-property-off-sale="putPropertyOffSale"
      :put-property-on-sale="putPropertyOnSale"
      :account-id="accountId"
  />
</template>

<script>
import TheHeader from '@/components/layouts/TheHeader';
import {useProperties, useWallet} from '@/composables/near';
import TheProperties from '@/components/TheProperties';

export default {
  name: 'TheHome',
  components: { TheProperties, TheHeader },
  setup() {
    const { accountId, signIn, signOut } = useWallet();
    const {
      properties,
      CONTRACT_ID,
      addProperty,
      deleteProperty,
      buyProperty,
      putPropertyOnSale,
      putPropertyOffSale
    } = useProperties();

    return {
      accountId,
      signIn,
      signOut,
      properties,
      CONTRACT_ID,
      addProperty,
      deleteProperty,
      buyProperty,
      putPropertyOnSale,
      putPropertyOffSale,
    }
  }
};
</script>

<style scoped>

</style>