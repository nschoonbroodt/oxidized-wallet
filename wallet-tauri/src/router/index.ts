import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "@/views/Dashboard.vue";
import Accounts from "@/views/Accounts.vue";
import { LayoutDashboard, Wallet, ArrowUpDown } from "lucide-vue-next";

declare module "vue-router" {
  interface RouteMeta {
    title?: string;
    icon?: any;
    showInNav?: boolean;
  }
}

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/dashboard",
    },
    {
      path: "/dashboard",
      component: Dashboard,
      meta: {
        title: "Tableau de bord",
        icon: LayoutDashboard,
        showInNav: true,
      },
    },
    {
      path: "/accounts",
      component: Accounts,
      meta: {
        title: "Comptes",
        icon: Wallet,
        showInNav: true,
      },
    },
    {
      path: "/transactions",
      component: () => import("@/views/Transactions.vue"),
      meta: {
        title: "Transactions",
        icon: ArrowUpDown,
        showInNav: true,
      },
    },
  ],
});
