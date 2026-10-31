<script setup lang="ts">
defineProps<{
  title: string;
  subtitle?: string;
  eyebrow?: string;
}>();
</script>

<template>
  <section class="base-page page-content-scroll">
    <div class="page-header-card">
      <div>
        <p v-if="eyebrow" class="eyebrow">{{ eyebrow }}</p>
        <h2>{{ title }}</h2>
        <p v-if="subtitle" class="muted-text">{{ subtitle }}</p>
      </div>
      <div v-if="$slots.actions" class="actions page-header-actions">
        <slot name="actions" />
      </div>
    </div>
    <div class="base-page-body">
      <slot />
    </div>
  </section>
</template>
