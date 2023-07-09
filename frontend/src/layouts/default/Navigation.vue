<script>
export default {
  name: 'Navigation',
  data() {
    return {
      selectedItem: 0,
      links: [
        {text: 'Dashboard', icon: 'mdi-view-dashboard', route: '/dashboard'},
        {text: 'Posts', icon: 'mdi-note-outline', route: '/posts'},
      ],
    }
  },
}
</script>

<template>
  <v-navigation-drawer
      app
      permanent>
    <v-list nav>
      <v-list-item
          v-for="(link, i) in links"
          :key="i"
          :value="link"
          color = "primary"
          :to="link.route"
          rounded="xl"
      >
        <template v-slot:prepend>
          <v-icon :icon="link.icon"/>
        </template>

        <v-list-item-title v-text="link.text"/>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<style>

</style>