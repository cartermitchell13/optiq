Add-Type -AssemblyName System.Drawing
$bmp = New-Object System.Drawing.Bitmap(512, 512)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.Clear([System.Drawing.Color]::FromArgb(233, 69, 96))
$font = New-Object System.Drawing.Font('Segoe UI', 200, [System.Drawing.FontStyle]::Bold)
$sf = New-Object System.Drawing.StringFormat
$sf.Alignment = [System.Drawing.StringAlignment]::Center
$sf.LineAlignment = [System.Drawing.StringAlignment]::Center
$rect = New-Object System.Drawing.RectangleF(0, 0, 512, 512)
$brush = [System.Drawing.Brushes]::White
$g.DrawString('P', $font, $brush, $rect, $sf)
$bmp.Save('C:\projects\optiq\app-icon.png', [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose()
$bmp.Dispose()
Write-Host "Icon created"
