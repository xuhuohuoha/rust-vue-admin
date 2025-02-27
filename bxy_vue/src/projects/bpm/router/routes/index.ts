export const bpmStaticRoutes = [
  {
    path: '/',
    name: 'Index',
    component: () => import('@bpm/views/IndexView.vue'),
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@bpm/views/LoginView.vue'),
  },
  {
    path: '/403',
    name: '403',
    component: () => import('@bpm/views/403View.vue'),
  },
  {
    path: '/500',
    name: '500',
    component: () => import('@bpm/views/500View.vue'),
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('@bpm/views/AboutView.vue'),
  },
  {
    path: '/home',
    name: 'Home',
    component: () => import('@bpm/views/HomeView.vue'),
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('@bpm/views/DashboardView.vue'),
  },
  {
    path: '/table',
    name: 'Table',
    component: () => import('@bpm/views/TableView.vue'),
  },
  {
    path: '/:pathMatch(.*)*',
    name: '404',
    component: () => import('@bpm/views/404View.vue'),
  },
]
