<script lang="ts">
  import RangeCalendar from "$lib/components/custom-range-calendar/wrapper/range-calendar.svelte";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import { CalendarDate, parseAbsoluteToLocal, parseDate, parseDateTime, parseZonedDateTime, type DateValue } from "@internationalized/date";
  import type { DateRange } from "bits-ui";
  import { MediaQuery } from "svelte/reactivity";
  import type { HighlightedRange } from "$lib/components/custom-range-calendar/base/types";
  import { isAfter, isBefore, isBetweenInclusive } from "$lib/components/custom-range-calendar/internal/date-time/utils";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { reservationDuration } from "$lib/utils";
    import Button from "./ui/button/button.svelte";
    import { invalidateAll } from "$app/navigation";

  let { user, reservations, numberOfMonths, value = $bindable({start: undefined, end: undefined}), disabled = false }: 
    { user: any, reservations: any[], numberOfMonths: number, value: DateRange, disabled?: boolean } = $props();

  // const isDesktop = new MediaQuery("(min-width: 768px)");

  // let numberOfMonths = $derived(isDesktop.current ? 5 : 2);

  let pendingReservationRemoval = $state(false);

  async function removeReservation(event: any, reservation: any) {
    event.stopPropagation();
    pendingReservationRemoval = true;
    let response = await fetch("/api/remove-reservation?id=" + reservation.id, { method: "POST" });
    invalidateAll();
    console.log(await response.text());
    pendingReservationRemoval = false;
  }

  let ranges: HighlightedRange[] = $derived(reservations.map(rsv => ({
      start: parseAbsoluteToLocal(rsv.start_date),
      end: parseAbsoluteToLocal(rsv.end_date),
      color: rsv.user.personal_color,
    })
  ));
  
  $effect(() => {
    // Reset selection if ranges overlap
    ranges.forEach(rsv => {
      if (value.start && value.end) {
        if (value.start < rsv.end && rsv.end < value.end) {
          value = {start:undefined, end: undefined};
        }
      }
      if (value.start && isBetweenInclusive(value.start, rsv.start, rsv.end)) {
        value = {start: undefined, end: undefined};
      }
    })
  })

  let popupOpen = $state(false);

  let focusedReservation: any = $state(undefined);

  function onClickHightlight(date: DateValue, highlight: number) {
    focusedReservation = reservations[highlight];
    popupOpen = true;
  }

</script>
<RangeCalendar bind:value class="rounded-lg" {ranges} learnMore={onClickHightlight} {disabled} fixedWeeks={false} {numberOfMonths}/>

<Dialog.Root bind:open={popupOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title> 
        <p>{reservationDuration(focusedReservation)}</p>
      </Dialog.Title>
    </Dialog.Header>
    {#if focusedReservation.user.username == user.username}
      <p>Denna reservation är gjord av dig</p>
      {#if pendingReservationRemoval}
        <LoaderCircleIcon/>
      {:else}
        <Button onclick={(e) => removeReservation(e, focusedReservation)} variant="destructive">Ta bort reservation</Button>
      {/if}
    {:else}
      <p>Denna reservation är gjord av <u>{focusedReservation.user.username}</u></p>
    {/if}
  </Dialog.Content>
</Dialog.Root>
