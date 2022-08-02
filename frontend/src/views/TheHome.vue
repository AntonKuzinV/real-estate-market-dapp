<template>
  <TheHeader :account-id="accountId" :sign-in="signIn" :sign-out="signOut" :add-property="addProperty"
             :update-properties="updateProperties"/>
  <TheProperties :properties="properties" :account-id="accountId" :update-properties="updateProperties"/>
</template>

<script>
import TheHeader from '@/components/layouts/TheHeader';
import TheProperties from '@/components/TheProperties';
import { useProperties, useWallet } from '@/composables/near';

export default {
  name: 'TheHome',
  components: { TheProperties, TheHeader },
  setup() {
    const { accountId, signOut, signIn } = useWallet();
    const { properties, updateProperties, addProperty } = useProperties();
    return {
      accountId,
      signOut,
      signIn,
      properties,
      updateProperties,
      addProperty
    };
  },
};
</script>

<style scoped>

</style>