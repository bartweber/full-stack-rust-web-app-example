<template>
  <v-app>
    <default-bar />

    <default-navigation />

    <default-view />
  </v-app>
</template>

<script lang="ts" setup>
  import DefaultBar from './AppBar.vue'
  import DefaultView from './View.vue'
  import DefaultNavigation from './Navigation.vue'
</script>
