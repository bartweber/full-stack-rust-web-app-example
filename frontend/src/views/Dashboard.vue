<script>
import {mapGetters} from "vuex";

export default {
  name: 'Dashboard',
  computed: {
    ...mapGetters('post', ['postCount']),
  },
  mounted() {
    this.$store.dispatch('post/fetchPosts')
  },
}

</script>

<template>
  <v-container>
    <v-row justify="start"
           align="center"
    >
      <v-col cols="auto">
        <v-btn size="small" variant="text" icon="mdi-refresh" @click="$store.dispatch('post/fetchPosts')"/>
      </v-col>
      <v-col cols="auto">
        <div>
          No. of posts: {{ postCount }}
        </div>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>

</style>