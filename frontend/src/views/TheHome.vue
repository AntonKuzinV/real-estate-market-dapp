<template>
  <TheHeader
      :accountId="accountId"
      :signIn="signIn"
      :signOut="signOut"
  />
</template>

<script>
import TheHeader from '@/components/layouts/TheHeader';
import {useProperties, useWallet} from '@/composables/near';

export default {
  name: 'TheHome',
  components: { TheHeader },
  setup() {
    const { accountId, signIn,  signOut } = useWallet();
    const { properties, CONTRACT_ID} = useProperties();

    return {
      accountId,
      signIn,
      signOut,
      properties,
      CONTRACT_ID
    }
  }
};
</script>

<style scoped>

</style>