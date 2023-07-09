<template>
  <div>
    <v-toolbar>
      <v-toolbar-items></v-toolbar-items>
      <v-toolbar-title>Posts</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn
          variant="flat"
          rounded="xl"
          color="primary"
          :to="{ name: 'posts.new'}"
      >
        Create
      </v-btn>
      <v-btn icon
             :disabled="selected.length === 0"
             @click="deleteSelected"
      >
        <v-badge :model-value="selected.length > 0" color="red" :content="selected.length">
          <v-icon>mdi-delete</v-icon>
        </v-badge>
      </v-btn>
      <v-btn
          :loading="isLoading"
          icon="mdi-refresh"
          @click="refresh"
      />
    </v-toolbar>

    <v-data-table-server
        :headers="headers"
        :items-length="totalPosts"
        :items="posts"
        item-value="id"
        show-select
        select-strategy="page"
        v-model="selected"
        :items-per-page="itemsPerPage"
        @update:options="fetchPosts"
    >
      <template v-slot:item.actions="{ item }">
        <v-menu>
          <template v-slot:activator="{ props }">
            <v-btn icon="mdi-dots-vertical"
                   variant="flat"
                   size="small"
                   :key="item.value"
                   v-bind="props"
            />
          </template>
          <v-list>
            <v-list-item
                :to=" { name: 'posts.edit' , params: { postId: item.value } }"
            >
              <v-list-item-title>Edit</v-list-item-title>
            </v-list-item>
            <v-list-item
                @click="deletePost(item.value)"
            >
              <v-list-item-title>Delete</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu>
      </template>
    </v-data-table-server>

    <router-view/>

  </div>
</template>

<script setup>
import {computed} from 'vue'
import {useStore} from 'vuex'
import { VDataTableServer } from 'vuetify/labs/VDataTable'

const store = useStore()

const isLoading = computed(() => store.getters['post/isLoading'])
const posts = computed(() => store.getters['post/posts'])
const totalPosts = computed(() => store.getters['post/totalPosts'])
const itemsPerPage = computed(() => store.getters['post/itemsPerPage'])
const selected = computed({
  get() {
    return store.getters['post/selected']
  },
  set(value) {
    store.commit('post/setSelected', value)
  },
})

const headers = [
  {title: 'Title', key: 'title'},
  {title: 'Text', key: 'text'},
  {title: '', key: 'actions', sortable: false, width: '70px'},
]

function refresh() {
  store.dispatch('post/fetchPosts')
}

async function fetchPosts({page, itemsPerPage}) {
  await store.commit('post/setPage', page)
  await store.commit('post/setItemsPerPage', itemsPerPage)
  await store.dispatch('post/fetchPosts')
}

function deletePost(id) {
  store.dispatch('post/deletePost', id)
}

function deleteSelected() {
  store.dispatch('post/deletePosts', selected.value)
}

</script>

<style>

</style>