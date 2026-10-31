<script setup lang="ts">
defineProps<{ actions: { title: string; route?: string; disabled?: boolean }[] }>();
</script>

<template>
  <section class="card quick-actions-card">
    <header class="section-card-header"><h3>Atalhos rápidos</h3></header>
    <div class="quick-actions-grid">
      <RouterLink v-for="action in actions" :key="action.title" :to="action.route || '/'" class="quick-action" :class="{ disabled: action.disabled }">
        {{ action.title }}
      </RouterLink>
    </div>
  </section>
</template>
