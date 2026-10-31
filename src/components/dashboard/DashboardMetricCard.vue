<script setup lang="ts">
defineProps<{ title: string; value: string | number; subtitle?: string; icon?: string; status?: 'neutral' | 'success' | 'warning' | 'error' | 'info' }>();
</script>

<template>
  <article class="dashboard-metric" :class="status || 'neutral'">
    <div class="metric-icon">{{ icon || '•' }}</div>
    <div>
      <strong>{{ title }}</strong>
      <span>{{ value }}</span>
      <p v-if="subtitle">{{ subtitle }}</p>
    </div>
  </article>
</template>
