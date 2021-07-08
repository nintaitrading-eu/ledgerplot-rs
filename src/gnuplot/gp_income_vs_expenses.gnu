set terminal pngcairo size 1920,1080 enhanced font 'Inconsolata,10'
  set output '/var/tmp/ledgerplot/yearly_income_vs_expenses.png'
  set style data histogram
  set style histogram clustered gap 1
  set style fill transparent solid 0.4 noborder
  set xtics nomirror scale 0 center
  set ytics add ('' 0) scale 0
  set border 1
  set grid ytics
  set title "Yearly Income and Expenses"
  set ylabel "Amount"
  plot "/var/tmp/ledgerplot/ledgeroutput1.tmp" using 2:xticlabels(strftime('%Y', strptime('%Y-%m-%d', strcol(1)))) title "Income" linecolor rgb "light-green", '' using 0:2:2 with labels left font "Inconsolata,8" rotate by 45 offset -4,0.5 textcolor linestyle 0 notitle, "/var/tmp/ledgerplot/ledgeroutput2.tmp" using 2 title "Expenses" linecolor rgb "light-salmon", '' using 0:2:2 with labels left font "Inconsolata,8" rotate by 45 offset 0,0.5 textcolor linestyle 0 notitle