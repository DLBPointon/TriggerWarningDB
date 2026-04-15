//
// THIS IS CLAUDE SONNET 4.6 GENERATED
// I STRUGGLED WITH THIS FOR A WHILE
//

'use strict';

window.addEventListener('load', function () {
    var dataEl = document.getElementById('heatmap-data');
    if (!dataEl) return;

    var payload;
    try {
        payload = JSON.parse(dataEl.textContent);
    } catch (e) {
        console.error('heatmap: failed to parse data', e);
        return;
    }

    var runtime = payload.runtime;
    var grouped = payload.events || [];

    if (!runtime || runtime <= 0) return;

    // ------------------------------------------------------------------
    // Build per-minute data
    // Deduplicate events by id — an event with multiple categories must
    // only be counted once per minute.
    // ------------------------------------------------------------------
    var minuteData = [];
    for (var i = 0; i < runtime; i++) {
        minuteData.push({ count: 0, events: [] });
    }

    var seen = {};
    for (var g = 0; g < grouped.length; g++) {
        var evts = grouped[g].events || [];
        for (var e = 0; e < evts.length; e++) {
            var evt = evts[e];
            if (seen[evt.id]) continue;
            seen[evt.id] = true;

            var start = Math.max(0, evt.time_minutes || 0);
            var dur = Math.max(1, evt.duration_minutes || 1);
            var end = Math.min(runtime, start + dur);

            for (var m = start; m < end; m++) {
                minuteData[m].count++;
                minuteData[m].events.push(evt);
            }
        }
    }

    var maxCount = 1;
    for (var i = 0; i < minuteData.length; i++) {
        if (minuteData[i].count > maxCount) maxCount = minuteData[i].count;
    }

    // ------------------------------------------------------------------
    // Draw canvas (DPR-aware for sharp rendering on retina screens)
    // ------------------------------------------------------------------
    var canvas = document.getElementById('heatmap-canvas');
    if (!canvas) return;

    var dpr = window.devicePixelRatio || 1;
    var displayW = canvas.parentElement.clientWidth;
    var displayH = 48;

    canvas.width = Math.round(displayW * dpr);
    canvas.height = Math.round(displayH * dpr);
    canvas.style.width = displayW + 'px';
    canvas.style.height = displayH + 'px';

    var ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);

    var blockW = displayW / runtime;

    // Colour scale:
    //   0 events  → #374151  (dim slate)
    //   low       → #f59e0b  (amber)
    //   max       → #ef4444  (red)
    function countToColor(count) {
        if (count === 0) return [55, 65, 81];
        var t = Math.min(1, count / maxCount);
        var r = Math.round(245 + t * (239 - 245));
        var gv = Math.round(158 + t * (68 - 158));
        var b = Math.round(11 + t * (68 - 11));
        return [r, gv, b];
    }

    for (var m = 0; m < runtime; m++) {
        var col = countToColor(minuteData[m].count);
        ctx.fillStyle = 'rgb(' + col[0] + ',' + col[1] + ',' + col[2] + ')';
        var gap = blockW > 3 ? 1 : 0;
        ctx.fillRect(m * blockW, 0, Math.max(1, blockW - gap), displayH);
    }

    // ------------------------------------------------------------------
    // Time markers
    // ------------------------------------------------------------------
    var markersEl = document.getElementById('heatmap-markers');
    if (markersEl) {
        var interval = runtime <= 60 ? 10 : runtime <= 120 ? 15 : 30;
        for (var m = 0; m <= runtime; m += interval) {
            var span = document.createElement('span');
            span.textContent = m + 'm';
            span.style.cssText = 'position:absolute;'
                + 'left:' + ((m / runtime) * 100) + '%;'
                + 'transform:translateX(-50%);'
                + 'font-size:0.7rem;'
                + 'color:#6b7280;'
                + 'white-space:nowrap;';
            markersEl.appendChild(span);
        }
    }

    // ------------------------------------------------------------------
    // Tooltip
    // ------------------------------------------------------------------
    var tooltip = document.getElementById('heatmap-tooltip');
    if (!tooltip) return;

    canvas.addEventListener('mousemove', function (e) {
        var rect = canvas.getBoundingClientRect();
        var minute = Math.floor(((e.clientX - rect.left) / rect.width) * runtime);

        if (minute < 0 || minute >= runtime) {
            tooltip.style.display = 'none';
            return;
        }

        var data = minuteData[minute];
        if (data.count === 0) {
            tooltip.style.display = 'none';
            return;
        }

        var html = '<strong>Minute ' + minute + '</strong><br>'
            + data.count + ' event' + (data.count !== 1 ? 's' : '') + ':<br>';

        for (var i = 0; i < data.events.length; i++) {
            var cats = (data.events[i].categories || []).join(', ') || 'Uncategorised';
            html += '<span style="color:#9ca3af">&bull; ' + cats + '</span><br>';
        }

        tooltip.innerHTML = html;

        // Flip left if too close to the right viewport edge
        var tw = tooltip.offsetWidth || 260;
        var left = (e.clientX + 14 + tw > window.innerWidth)
            ? e.clientX - tw - 14
            : e.clientX + 14;

        tooltip.style.left = left + 'px';
        tooltip.style.top = (e.clientY - 8) + 'px';
        tooltip.style.display = 'block';
    });

    canvas.addEventListener('mouseleave', function () {
        tooltip.style.display = 'none';
    });
});
