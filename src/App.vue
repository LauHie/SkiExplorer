<script setup lang="ts">
import { ref, computed } from "vue";
import MapView from "./components/Map.vue";
import { loadSkiAreas } from "./services/skiService";
import { Position } from "./types/position";
import { Slider } from "@/components/ui/slider";
import { Button } from "@/components/ui/button";
import { useTauriDesktopGuards } from "./composables/useTauriDesktopGuards";
import { Spinner } from "@/components/ui/spinner";
import { loadStandort, Standort } from "./services/standortService";
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet";
import {
  Mountain,
  MapPin,
  Search,
  List,
  AlertCircle,
  ChevronRight,
  Ruler,
} from "lucide-vue-next";

useTauriDesktopGuards({
  blockFileDrop: true,
  blockContextMenu: true,
  blockContextMenuInDev: false,
});

const mapRef = ref<InstanceType<typeof MapView>>();
const posRef = ref<Position>({ lat: 52.52437, lon: 13.41053 });
const sliderRef = ref<number[] | undefined>([5]);
const errorRef = ref<string | null>(null);
const waitingForApi = ref(false);
const searchingRef = ref(false);
const searchRef = ref<string>("");
const isOpen = ref(false);
const standorteRef = ref<Standort[]>([]);
const selectedStandort = ref<Standort | null>(null);
const lastQueryRef = ref<string>("");

const hasResults = computed(() => standorteRef.value.length > 0);

async function getSkiAreas(pos: Position) {
  try {
    if (sliderRef.value) {
      waitingForApi.value = true;
      const pistes = await loadSkiAreas(pos, sliderRef.value[0] * 1000);
      waitingForApi.value = false;
      mapRef.value?.addPisteMarkers(pistes);
      errorRef.value = null;
    }
  } catch (error: unknown) {
    waitingForApi.value = false;
    errorRef.value =
      error instanceof Error ? error.message : "Unbekannter Fehler";
  }
}

async function getStandort() {
  const query = searchRef.value.trim();
  if (!query) {
    errorRef.value = "Fehlende Eingabe";
    return;
  }
  try {
    searchingRef.value = true;
    standorteRef.value = await loadStandort(query);
    lastQueryRef.value = query;
    selectedStandort.value = null;
    errorRef.value = null;
    mapRef.value?.showStandorte(standorteRef.value);
    isOpen.value = true;
  } catch (error: unknown) {
    standorteRef.value = [];
    errorRef.value =
      error instanceof Error ? error.message : "Suche fehlgeschlagen";
  } finally {
    searchingRef.value = false;
  }
}

function selectStandort(standort: Standort) {
  selectedStandort.value = standort;
  posRef.value = standort.pos;
  mapRef.value?.setView(standort.pos);
  mapRef.value?.showStandorte(standorteRef.value, standort.id);
}

function onPositionChanged(pos: Position) {
  posRef.value = pos;
}

function openResults() {
  if (hasResults.value) isOpen.value = true;
}

function clearResults() {
  standorteRef.value = [];
  selectedStandort.value = null;
  lastQueryRef.value = "";
  mapRef.value?.clearStandorte();
  isOpen.value = false;
}
</script>

<template>
  <div
    class="relative flex h-screen w-screen overflow-hidden bg-background font-sans text-foreground antialiased"
  >
    <!-- Sidebar -->
    <aside
      class="z-10 flex w-[320px] shrink-0 flex-col justify-between border-r border-border bg-sidebar shadow-xl"
    >
      <div class="flex flex-1 flex-col gap-7 overflow-y-auto px-6 py-7">
        <!-- Logo / Brand -->
        <div class="flex items-center gap-3">
          <div
            class="flex h-11 w-11 items-center justify-center rounded-xl bg-primary text-primary-foreground shadow-sm"
          >
            <Mountain class="size-6" />
          </div>
          <div class="flex flex-col">
            <h1 class="text-lg font-semibold leading-tight tracking-tight">
              Ski Explorer
            </h1>
            <p class="text-xs text-muted-foreground">
              Finde Pisten in deiner Nähe
            </p>
          </div>
        </div>

        <!-- Search section -->
        <section class="flex flex-col gap-3">
          <div class="flex items-center gap-2">
            <Search class="size-4 text-muted-foreground" />
            <label
              for="search-input"
              class="text-xs font-medium uppercase tracking-wider text-muted-foreground"
            >
              Standort suchen
            </label>
          </div>

          <div class="relative">
            <input
              id="search-input"
              v-model="searchRef"
              type="text"
              autocomplete="off"
              autocorrect="off"
              autocapitalize="off"
              spellcheck="false"
              placeholder="z.B. Berlin, Innsbruck …"
              class="w-full rounded-md border border-border bg-background px-3 py-2 pr-10 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
              @keydown.enter="getStandort"
            />
            <MapPin
              class="pointer-events-none absolute right-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
            />
          </div>

          <Button class="w-full" :disabled="searchingRef" @click="getStandort">
            <Spinner v-if="searchingRef" />
            <template v-else>
              <Search class="size-4" />
              <span>Suchen</span>
            </template>
          </Button>

          <Button
            v-if="hasResults"
            variant="outline"
            class="w-full justify-between"
            @click="openResults"
          >
            <span class="flex items-center gap-2">
              <List class="size-4" />
              Ergebnisse ({{ standorteRef.length }})
            </span>
            <ChevronRight class="size-4" />
          </Button>
        </section>

        <div class="h-px w-full bg-border" />

        <!-- Radius / Pisten section -->
        <section class="flex flex-col gap-4">
          <div class="flex items-center gap-2">
            <Ruler class="size-4 text-muted-foreground" />
            <label
              class="text-xs font-medium uppercase tracking-wider text-muted-foreground"
            >
              Suchradius
            </label>
          </div>

          <div class="flex flex-col gap-2">
            <Slider
              :value="sliderRef"
              @update:modelValue="sliderRef = $event"
              :max="500"
              :step="5"
            />
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground">0 km</span>
              <span
                class="rounded-md bg-secondary px-2 py-0.5 font-semibold text-secondary-foreground tabular-nums"
              >
                {{ sliderRef?.[0] ?? 0 }} km
              </span>
              <span class="text-muted-foreground">500 km</span>
            </div>
          </div>

          <Button
            variant="default"
            class="w-full"
            :disabled="waitingForApi"
            @click="getSkiAreas(posRef)"
          >
            <Spinner v-if="waitingForApi" />
            <template v-else>
              <Mountain class="size-4" />
              <span>Pisten laden</span>
            </template>
          </Button>
        </section>

        <!-- Error message -->
        <div
          v-if="errorRef"
          class="flex items-start gap-2 rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
        >
          <AlertCircle class="mt-0.5 size-4 shrink-0" />
          <span class="leading-snug">{{ errorRef }}</span>
        </div>

        <!-- Selected standort card -->
        <div
          v-if="selectedStandort"
          class="mt-auto rounded-lg border border-border bg-card p-3 shadow-sm"
        >
          <div class="flex items-start gap-2">
            <MapPin class="mt-0.5 size-4 shrink-0 text-primary" />
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-semibold text-card-foreground">
                {{ selectedStandort.name }}
              </p>
              <p class="truncate text-xs text-muted-foreground">
                {{ selectedStandort.class }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <footer class="border-t border-border px-6 py-4 text-center">
        <p class="text-xs text-muted-foreground">© 2026 Ski Explorer</p>
      </footer>
    </aside>

    <!-- Main content -->
    <main class="relative h-full flex-1 bg-muted/30 p-4">
      <MapView
        ref="mapRef"
        class="h-full w-full overflow-hidden rounded-xl border border-border shadow-lg"
        :pos="posRef"
        @position-changed="onPositionChanged"
      />
    </main>

    <!-- Search results sheet -->
    <Sheet v-model:open="isOpen">
      <SheetContent side="right" class="sm:max-w-md">
        <SheetHeader class="gap-1">
          <SheetTitle class="flex items-center gap-2 text-lg">
            <List class="size-5" />
            Suchergebnisse
          </SheetTitle>
          <SheetDescription>
            <span v-if="hasResults">
              {{ standorteRef.length }} Treffer für
              <span class="font-medium text-foreground">
                „{{ lastQueryRef }}"
              </span>
            </span>
            <span v-else>Keine Ergebnisse.</span>
          </SheetDescription>
        </SheetHeader>

        <div class="flex flex-1 flex-col gap-2 overflow-y-auto px-4 pb-2">
          <button
            v-for="(standort, i) in standorteRef"
            :key="standort.id"
            type="button"
            @click="selectStandort(standort)"
            :class="[
              'group flex items-start gap-3 rounded-lg border p-3 text-left transition-colors',
              selectedStandort?.id === standort.id
                ? 'border-primary bg-primary/5 ring-1 ring-primary/30'
                : 'border-border hover:border-primary/40 hover:bg-accent',
            ]"
          >
            <span
              :class="[
                'flex size-7 shrink-0 items-center justify-center rounded-full text-xs font-semibold shadow-sm',
                selectedStandort?.id === standort.id
                  ? 'bg-primary text-primary-foreground'
                  : 'bg-secondary text-secondary-foreground group-hover:bg-primary group-hover:text-primary-foreground',
              ]"
            >
              {{ i + 1 }}
            </span>
            <div class="min-w-0 flex-1">
              <p class="truncate font-medium leading-snug">
                {{ standort.name }}
              </p>
              <p class="truncate text-xs text-muted-foreground">
                {{ standort.class }}
              </p>
              <p class="mt-1 text-[11px] tabular-nums text-muted-foreground/80">
                {{ standort.pos.lat.toFixed(4) }},
                {{ standort.pos.lon.toFixed(4) }}
              </p>
            </div>
            <ChevronRight
              class="mt-1 size-4 shrink-0 text-muted-foreground transition-transform group-hover:translate-x-0.5"
            />
          </button>
        </div>

        <SheetFooter class="border-t border-border pt-4">
          <Button variant="ghost" class="w-full" @click="clearResults">
            Ergebnisse löschen
          </Button>
          <SheetClose as-child>
            <Button variant="outline" class="w-full">Schließen</Button>
          </SheetClose>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  </div>
</template>
