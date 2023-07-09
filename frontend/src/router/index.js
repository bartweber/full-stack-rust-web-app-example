import {createRouter, createWebHistory} from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: () => import('@/layouts/default/Default.vue'),
            children: [
                {
                    path: 'dashboard',
                    name: 'dashboard',
                    component: () => import('@/views/Dashboard.vue'),
                },
                {
                    path: 'posts',
                    name: 'posts',
                    component: () => import('@/views/PostsView.vue'),
                    children: [
                        {
                            path: 'new',
                            name: 'posts.new',
                            component: () => import('@/components/PostDialog.vue')
                        },
                        {
                            path: ':postId/edit',
                            name: 'posts.edit',
                            component: () => import('@/components/PostDialog.vue')
                        },
                    ],
                },
            ]
        },
    ]
})

export default router
