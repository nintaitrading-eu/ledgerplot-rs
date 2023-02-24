set terminal pngcairo size 1920,1080 enhanced font 'Inconsolata,10'
set output 'yearly_wealthgrowth.png'
#set xtics nomirror "" ,31104000 format "%Y"
#set xtics offset 0 nomirror "" ,31104000 format "%Y-%m-%d"
#set xtics offset 0 nomirror "" ,365 format "%Y-%m"
set xtics format "%Y"
#set xtics rotate by 60 right
#set xtics "2008-01-01", 31449600, "2020-12-31"
set mxtics 2
set mytics 2
set grid xtics mxtics ytics mytics
set title "Wealthgrowth"
set ylabel "Amount"
set style fill transparent solid 0.6 noborder
plot "/tmp/ledgerplot/ledgeroutput1.tmp" using 1:2 with filledcurves x1 title "Assets" linecolor rgb "light-green", "/tmp/ledgerplot/ledgeroutput2.tmp" using 1:2 with filledcurves y1=0 title "Liabilities" linecolor rgb "light-salmon"
