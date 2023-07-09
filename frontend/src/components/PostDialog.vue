<template>
  <v-dialog
      max-width="500px"
      persistent
      v-model="dialog"
  >
    <v-card rounded="xl" class="pa-3">
      <v-card-title>Create Post</v-card-title>
      <v-form ref="form" v-on:submit.prevent="save">
        <v-card-text>
          <v-text-field
              ref="title"
              autofocus
              v-model="formData.title"
              label="Title *"
              :rules="nameRules"
              required
              variant="underlined"
          ></v-text-field>
          <v-textarea
              v-model="formData.text"
              label="Text"
              required
              variant="underlined"
          ></v-textarea>
        </v-card-text>
      </v-form>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
            @click="cancel"
            rounded
            class="text-none"
        >Cancel
        </v-btn>
        <v-btn
            class="text-none"
            type="submit"
            color="primary"
            @click="save"
            rounded
            variant="outlined"
        >
          Save
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import {ref, onMounted} from 'vue'
import {useRouter, useRoute} from "vue-router";
import {useStore} from "vuex";

const dialog = true
const store = useStore()
const router = useRouter()
const route = useRoute()

const formData = ref({
  id: '',
  title: '',
  text: '',
})

const form = ref(null)
const title = ref(null)

const id = route.params.postId
const isNewPost = id === undefined

const nameRules = [
  value => {
    if (value) return true

    return 'Name is required.'
  },
  value => {
    if (value?.length >= 3) return true

    return 'Name must be 3 or more characters long.'
  },
]

async function save() {
  const {valid} = await form.value.validate()
  // const valid = true

  if (valid) {
    console.log('valid!')
    if (isNewPost) {
      await store.dispatch('post/addPost', formData.value)
    } else {
      await store.dispatch('post/updatePost', formData.value)
    }
    await router.push({name: 'posts'})
  } else {
    console.log('invalid')
  }
}

function cancel() {
  console.log('canceled')
  router.push({name: 'posts'})
}

onMounted(async () => {
  if (!isNewPost) {
    console.log('fetch post with id: ' + id)
    await store.dispatch('post/fetchPost', id)
        .then(data => {
          formData.value = data
        })
        .catch(reason => {
          console.log(reason)
        })
  } else {
    console.log('starting new post')
  }
})

</script>

<style>

</style>