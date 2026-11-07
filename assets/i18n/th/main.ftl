# common
common-on = เปิด
common-off = ปิด
common-left = ซ้าย
common-right = ขวา
common-search = ค้นหา
common-unknown = ไม่ทราบ
common-not-provided = ไม่มีให้
common-not-available = ไม่สามารถใช้ได้
common-cancel = ยกเลิก
common-save = บันทึก
common-delete = ลบ
common-play = เล่น
common-more = เพิ่มเติม
common-previous = ก่อนหน้า
common-next = ถัดไป
common-dismiss = ปิด
common-clear = ล้าง
option-variants = ตัวเลือกของ { $name }
table-sorted-ascending = เรียงลำดับจากน้อยไปหามาก
table-sorted-descending = เรียงลำดับจากมากไปน้อย
range-from = จาก
range-to = ถึง
rail-back = เลื่อนกลับ
rail-forward = เลื่อนไปข้างหน้า
number-group = { "," }
# navigation
nav-history = ประวัติการเล่น
nav-home = หน้าแรก
nav-search = ค้นหา
nav-library = คลังเพลง
nav-settings = ตั้งค่า
nav-songs = เพลง
nav-albums = อัลบั้ม
nav-playlists = เพลย์ลิสต์
nav-artists = ศิลปิน
nav-local = เพลงในเครื่อง
nav-back = กลับ
nav-forward = ไปข้างหน้า
nav-sidebar = สลับแถบด้านข้าง
nav-sidebar-right = แสดงหรือซ่อนเนื้อเพลงและคิว
nav-pinned = ปักหมุด
nav-unpin = เลิกปักหมุด
nav-pin-hint = ลากมาวางที่นี่เพื่อปักหมุด
library-liked-songs = รายการโปรด
library-no-songs = ยังไม่มีรายการโปรด
library-no-albums = ยังไม่มีอัลบั้มที่บันทึกไว้
library-no-playlists = ยังไม่มีเพลย์ลิสต์
library-no-artists = ยังไม่มีศิลปินที่ชื่นชอบ
library-no-local-songs = ไม่พบเพลงที่นำเข้า
library-no-local-albums = ไม่พบอัลบั้มที่นำเข้า
library-no-local-artists = ไม่พบศิลปินจากเพลงที่นำเข้า
library-no-local-playlists = ยังไม่มีเพลย์ลิสต์ในเครื่อง
library-no-catalog-songs = ไม่พบเพลง
library-no-catalog-albums = ไม่พบอัลบั้ม
library-no-catalog-artists = ไม่พบศิลปิน
library-no-matches = ไม่พบรายการที่ตรงกัน
library-not-loaded = โหลดคลังเพลงไม่ได้
library-scanning = กำลังค้นหาเพลงในโฟลเดอร์…
library-scanning-found = กำลังค้นหาเพลงในโฟลเดอร์… พบแล้ว { $found } เพลง
library-scanning-progress = กำลังอ่านเพลงของคุณ… { $read } จาก { $found }
library-part-not-loaded = โหลดคลังเพลงส่วนนี้ไม่ได้
library-local-unconfigured = ตั้งค่าคลังเพลงในเครื่อง
# app menu
app-refresh = รีเฟรช
downloads-active = กำลังดาวน์โหลด
downloads-left =
    { $count ->
        [one] เหลือเพลง { $count } แล้ว
       *[other] เหลือเพลง { $count } แล้ว
    }
downloads-eta = เหลืออีกประมาณ { $minutes } นาที
downloads-songs = เพลง
downloads-cancel = ยกเลิกการดาวน์โหลด
downloads-cancel-all = ยกเลิกการดาวน์โหลดทั้งหมด
app-sign-out = ออกจากระบบ
app-quit = ออกจากแอป
app-settings = การตั้งค่า…
app-hide = ซ่อน SonoraX
app-hide-others = ซ่อนแอปอื่น
app-show-all = แสดงทั้งหมด
app-edit = แก้ไข
app-cut = ตัด
app-copy = คัดลอก
app-paste = วาง
app-select-all = เลือกทั้งหมด
app-window = หน้าต่าง
app-close-window = ปิดหน้าต่าง
app-minimize = ย่อเล็กสุด
app-zoom = ซูม
window-maximize = ขยายใหญ่สุด
window-restore = คืนค่า
# tray menu
tray-show = แสดง SonoraX
tray-play = เล่น
tray-pause = หยุดชั่วคราว
# table columns
column-played-at = เล่นแล้ว
column-index = #
column-title = ชื่อเพลง
column-artist = ศิลปิน
column-album = อัลบั้ม
column-date-added = วันที่เพิ่ม
column-added-by = เพิ่มโดย
column-modified = แก้ไขล่าสุด
column-length = ความยาว
column-plays = จำนวนครั้งที่เล่น
column-name = ชื่อ
column-owner = เจ้าของ
column-year = ปี
column-tracks = แทร็ก
# track menu
menu-add-to-playlist = เพิ่มลงในเพลย์ลิสต์
menu-add-tracks-to-playlist =
    { $count ->
        [one] เพิ่มแทร็ก { $count } ลงในเพลย์ลิสต์
       *[other] เพิ่มแทร็ก { $count } ลงในเพลย์ลิสต์
    }
menu-new-playlist = เพลย์ลิสต์ใหม่
menu-import-playlist = นำเข้าเพลย์ลิสต์...
menu-edit-tags = แก้ไขแท็ก
menu-no-playlists = ไม่มีเพลย์ลิสต์
menu-search-playlists = ค้นหาเพลย์ลิสต์
menu-save-library = คลังเพลง
menu-add-to-library = เพิ่มในรายการโปรด
menu-remove-from-library = ลบออกจากรายการโปรด
menu-remove-from-playlist = ลบออกจากเพลย์ลิสต์
menu-remove-tracks-from-playlist =
    { $count ->
        [one] ลบแทร็ก { $count } ออกจากเพลย์ลิสต์
       *[other] ลบแทร็ก { $count } ออกจากเพลย์ลิสต์
    }
menu-remove-from-history = ลบออกจากประวัติ
menu-remove-tracks-from-history =
    { $count ->
        [one] ลบแทร็ก { $count } ออกจากประวัติ
       *[other] ลบแทร็ก { $count } ออกจากประวัติ
    }
menu-delete-track-file = ลบไฟล์แทร็ก
menu-delete-track-files = ลบไฟล์แทร็ก { $count }
menu-download = ดาวน์โหลด
menu-remove-download = ลบการดาวน์โหลด
menu-downloading = กำลังดาวน์โหลด คลิกเพื่อหยุด
menu-play-next = เล่นเป็นเพลงถัดไป
menu-play-tracks-next =
    { $count ->
        [one] เล่นแทร็ก { $count } ถัดไป
       *[other] เล่นแทร็ก { $count } ถัดไป
    }
menu-add-to-queue = เพิ่มในคิว
menu-add-tracks-to-queue =
    { $count ->
        [one] เพิ่มแทร็ก { $count } ลงในคิว
       *[other] เพิ่มแทร็ก { $count } ลงในคิว
    }
menu-play-last = เล่นครั้งสุดท้าย
menu-play-tracks-last =
    { $count ->
        [one] เล่นเพลง { $count } เป็นเพลงสุดท้าย
       *[other] เล่นแทร็ก { $count } เป็นลำดับสุดท้าย
    }
menu-song-radio = ไปที่วิทยุเพลง
menu-go-to-album = ไปที่อัลบั้ม
menu-go-to-artist = ไปหาศิลปิน
menu-view-details = ดูรายละเอียด
menu-copy-link = คัดลอกลิงก์
menu-cut = ตัด
menu-copy = คัดลอก
menu-paste = วาง
menu-select-all = เลือกทั้งหมด
menu-remove-from-queue = เอาออกจากคิว
menu-edit-playlist = แก้ไขเพลย์ลิสต์
menu-delete-playlist = ลบเพลย์ลิสต์
menu-export-playlist = ส่งออกเพลย์ลิสต์...
menu-add-playlist-to-library = เพิ่มลงในคลังเพลง
menu-remove-playlist-from-library = ลบออกจากคลังเพลง
menu-make-playlist-public = ทำให้เป็นสาธารณะ
menu-make-playlist-private = ทำให้เป็นส่วนตัว
menu-play-artist = เล่นศิลปิน
# playlist editor
playlist-name-placeholder = ชื่อเพลย์ลิสต์
playlist-create-title = สร้างเพลย์ลิสต์
playlist-edit-title = แก้ไขเพลย์ลิสต์
playlist-cover-choose = เลือกรูปภาพ
playlist-cover-remove = ถอดฝาครอบออก
playlist-cover-unopenable = SonoraXไม่สามารถเปิดไฟล์นี้ได้
playlist-cover-too-large = รูปภาพนี้มีขนาดใหญ่กว่า 40 MB
playlist-cover-not-a-picture = ไฟล์นี้ไม่ใช่รูปภาพ JPEG, PNG หรือ WebP
playlist-delete-title = ลบเพลย์ลิสต์
playlist-delete-confirm = ลบ “{ $name }” หรือไม่ สิ่งนี้ไม่สามารถยกเลิกได้
playlist-again-title = เพิ่มอีกครั้ง?
playlist-again-confirm = แทร็กนี้มีอยู่ใน “{ $name }” แล้ว เพิ่มสำเนาอื่นหรือไม่?
playlist-again-add = เพิ่มอีกครั้ง
# confirm
confirm-remove-library-title = ลบออกจากคลังเพลง
confirm-remove-playlist-title = ลบออกจากเพลย์ลิสต์
confirm-remove-history-title = ลบออกจากประวัติ
confirm-remove-songs =
    { $count ->
        [one] ลบเพลงนี้ออกจากคลังเพลงของคุณใช่ไหม
       *[other] ลบเพลง { $count } ออกจากคลังเพลงของคุณหรือไม่?
    }
confirm-remove-playlist-songs =
    { $count ->
        [one] ลบเพลงนี้ออกจากเพลย์ลิสต์ใช่ไหม
       *[other] ลบเพลง { $count } ออกจากเพลย์ลิสต์หรือไม่
    }
confirm-remove-history-songs =
    { $count ->
        [one] ลบเพลงนี้ออกจากประวัติการฟังไหม
       *[other] ลบเพลง { $count } ออกจากประวัติการฟังหรือไม่
    }
confirm-remove-albums =
    { $count ->
        [one] ลบอัลบั้มนี้ออกจากคลังเพลงของคุณหรือไม่?
       *[other] ลบอัลบั้ม { $count } ออกจากคลังเพลงของคุณหรือไม่?
    }
confirm-remove-artists =
    { $count ->
        [one] ลบศิลปินนี้ออกจากรายการโปรดหรือไม่
       *[other] ลบศิลปิน { $count } ออกจากรายการโปรดหรือไม่
    }
confirm-remove-playlists =
    { $count ->
        [one] ลบเพลย์ลิสต์นี้ออกจากคลังเพลงของคุณใช่ไหม
       *[other] ลบเพลย์ลิสต์ { $count } ออกจากคลังเพลงของคุณหรือไม่
    }
confirm-delete-track-files-title = ลบไฟล์แทร็กใช่ไหม
confirm-delete-track-files = ไฟล์จะถูกลบออกจากดิสก์อย่างถาวร สิ่งนี้ไม่สามารถยกเลิกได้
# queue panel
queue-title = คิวเพลง
queue-history = ประวัติการเล่น
queue-now-playing = กำลังเล่น
queue-from = จาก
queue-from-radio = วิทยุ { $name }
queue-next-in-queue = ถัดไปในคิว
queue-up-next = ต่อไป
queue-reset = รีเซ็ต
queue-clear = ล้างคิว
queue-added-by = เพิ่มโดย { $name }
queue-empty = คิวว่าง
queue-similar = เพลงคล้ายกัน
queue-radio = เล่นแทร็กที่คล้ายกันโดยอัตโนมัติ
queue-return-playing = กลับมาเล่นต่อแล้ว
# player bar
player-nothing-playing = ยังไม่เล่น
player-percent = { $value }%
player-shuffle = สุ่มเพลง
player-repeat = ทำซ้ำ
player-repeat-all = วนทั้งหมด
player-repeat-one = วนเพลงเดียว
player-mute = ปิดเสียง
player-unmute = เปิดเสียง
player-previous = เพลงก่อนหน้า
player-next = เพลงถัดไป
player-fullscreen = เต็มจอ
player-fullscreen-leave = ออกจากเต็มจอ
devices-this-computer = คอมพิวเตอร์เครื่องนี้
devices-jam-join = เข้าร่วม Jam...
devices-jam-join-title = เข้าร่วม Jam
devices-jam-join-detail = วางลิงก์ที่เพื่อนแชร์กับคุณ
devices-jam-paste = วาง
devices-jam-link-hint = ลิงค์แจม
devices-jam-start = เริ่ม Jam
devices-jam-copy = คัดลอกลิงก์ Jam
devices-jam-code-menu = แสดงรหัส QR
devices-jam-code-title = สแกนเพื่อเข้าร่วม Jam ของ { $host }
devices-jam-code-title-anonymous = สแกนเพื่อเข้าร่วม Jam
devices-jam-code-close = ปิด
devices-jam-leave = ทิ้งแจม
devices-jam-end = จบแจม
devices-jam-more = +{ $count }
devices-jam-host = โฮสต์
devices-jam-you = คุณ
devices-jam-remove = ลบ
devices-jam-people =
    { $count ->
        [one] { $count } คนใน Jam นี้
       *[other] คน { $count } ใน Jam นี้
    }
devices-jam-remove-member = ลบ { $name } ออก
devices-jam-invite-title = เข้าร่วม Jam ของ { $host } ไหม?
devices-jam-invite-title-anonymous = เข้าร่วม Jam นี้ไหม
devices-jam-invite-detail =
    { $count ->
        [one] มีบุคคล { $count } อยู่ใน Jam นี้แล้ว
       *[other] ชาว { $count } อยู่ใน Jam นี้แล้ว
    }
devices-jam-invite-join = เข้าร่วม
player-sessions = เซสชัน
devices-spotify-jams = Spotify แยม
members-title = สมาชิก
members-empty = เริ่มหรือเข้าร่วมเซสชั่นเพื่อดูว่าใครกำลังฟังคุณอยู่
devices-session-start = เริ่มเซสชั่น
devices-session-join = เข้าร่วมเซสชั่น...
devices-session-join-title = เข้าร่วมเซสชัน
devices-session-join-detail = ป้อนรหัสที่เพื่อนแชร์กับคุณ
devices-session-join-confirm = เข้าร่วม
devices-session-code = รหัสเซสชัน
devices-session-code-menu = แสดงรหัสเซสชัน
devices-session-code-title = รหัสเซสชันของคุณ
devices-session-code-detail = ทุกคนที่ลงชื่อเข้าใช้ { $provider } สามารถเข้าร่วมด้วยรหัสนี้
devices-session-code-close = เสร็จแล้ว
devices-session-copy = คัดลอกรหัส
devices-session-leave = ออกจากเซสชัน
devices-session-end = สิ้นสุดเซสชัน
devices-session-reconnecting = กำลังเชื่อมต่อใหม่...
devices-session-people =
    { $count ->
        [one] บุคคล { $count } ในเซสชันนี้
       *[other] คน { $count } ในเซสชันนี้
    }
fullscreen-artwork = งานศิลปะ
fullscreen-panels = กำลังเล่นมุมมอง
# filters
filter-history = กรองประวัติการฟัง
history-empty = เพลงที่คุณเล่นจะปรากฏที่นี่
history-not-loaded = ไม่สามารถโหลดประวัติการฟังได้
history-clear = ล้างประวัติ
history-clear-title = ล้างประวัติการฟัง
history-clear-confirm = การเล่นทุกครั้งจะถูกลบออกจากอุปกรณ์นี้ สิ่งนี้ไม่สามารถยกเลิกได้
filter-library = กรองคลังเพลงของคุณ
filter-album = กรองแทร็กอัลบั้ม
filter-reset = รีเซ็ตตัวกรอง
filter-duration = ระยะเวลา
filter-year = ปี
filter-hide-explicit = ซ่อนอย่างโจ่งแจ้ง
filter-playable = เล่นได้เท่านั้น
filter-favorites = รายการโปรดเท่านั้น
filter-downloaded = ดาวน์โหลดแล้ว
filter-owned = โดยคุณ
# view
view-list = รายการ
view-cards = กริด
# toolbar
tool-sort = จัดเรียง
tool-filters = ตัวกรอง
# login
login-signed-out = ลงชื่อเข้าใช้เพื่อโหลดคลังเพลงของคุณ
login-restoring = กำลังตรวจสอบเซสชันที่บันทึกไว้...
login-authorizing = กำลังรอการอนุญาตในเบราว์เซอร์ของคุณ...
login-signed-in = ลงชื่อเข้าใช้เป็น { $name }
login-failed-title = การลงชื่อเข้าใช้ล้มเหลว
login-problem-region = Spotify จะไม่เปิดเซสชันจากประเทศที่คุณอยู่ ลงชื่อเข้าใช้จากประเทศบ้านเกิดของคุณ หรือเปลี่ยนประเทศในบัญชี Spotify ของคุณ
login-problem-credentials = เซสชันที่คุณบันทึกไว้สำหรับบริการนี้ใช้ไม่ได้อีกต่อไป ลงชื่อเข้าใช้อีกครั้งเพื่อดำเนินการต่อ
login-problem-network = SonoraXไม่สามารถเข้าถึงบริการเพลงได้ ตรวจสอบการเชื่อมต่ออินเทอร์เน็ตของคุณแล้วลองอีกครั้ง
login-problem-cancelled = การลงชื่อเข้าใช้ไม่ได้รับการอนุมัติในเบราว์เซอร์หรือหมดเวลา เริ่มต้นใหม่อีกครั้งให้จบ
login-problem-refused = Spotify ปิดการลงชื่อเข้าใช้ รอสักครู่แล้วลองอีกครั้ง
login-problem-premium = SonoraX สตรีมผ่าน Spotify Premium และบัญชีนี้ไม่มี ลงชื่อเข้าใช้ด้วยบัญชีพรีเมียมเพื่อดำเนินการต่อ
login-problem-subscription = ไม่มีการสมัครสมาชิกบน Apple ID นี้
login-subscription-check = ตรวจสอบแอปเปิ้ลมิวสิค
login-sign-in = เข้าสู่ระบบด้วย { $provider }
login-connect-cookies = วางคุกกี้ด้วยตนเอง
login-cookie-open = เปิด { $provider }
login-cookie-submit = ดำเนินการต่อ
login-cookie-hint = วางส่วนหัวคำขอคุกกี้ที่นี่
login-cookie-step-1 = เปิด music.youtube.com และตรวจสอบว่าคุณได้ลงชื่อเข้าใช้แล้ว หน้าต่างที่ไม่ระบุตัวตนจะทำงานได้ดีที่สุด
login-cookie-step-2 = กด F12 เปิดแท็บเครือข่ายแล้วโหลดหน้าซ้ำ
login-cookie-step-3 = เลือกคำขอชื่อ "เรียกดู" หรือ "ถัดไป"
login-cookie-step-4 = ในส่วนหัว ให้ค้นหาคุกกี้ใต้คำขอส่วนหัว คลิกขวาแล้วคัดลอกค่า
login-cookie-step-note = ตรวจสอบให้แน่ใจว่าได้วางค่าทั้งหมด รวมถึง SAPISID และ __Secure-3PAPISID
login-cookie-header-title = วางคุกกี้ { $provider } ของคุณเพื่อลงชื่อเข้าใช้ให้เสร็จสิ้น
login-cookie-apple-step-3 = เลือกคำขอใด ๆ ไปยัง { $site }
login-cookie-apple-note = ตรวจสอบให้แน่ใจว่าได้วางค่าทั้งหมด รวมถึง { $cookie } ด้วย
login-cookie-named-title = วางคุกกี้ { $provider } ของคุณเพื่อลงชื่อเข้าใช้ให้เสร็จสิ้น
login-cookie-named-hint = วางค่าคุกกี้ที่นี่
login-cookie-named-step-1 = เปิด { $site } และตรวจสอบว่าคุณได้ลงชื่อเข้าใช้แล้ว
login-cookie-named-step-2 = กด F12 แล้วเปิดแท็บ Storage ใน Firefox หรือแท็บ Application ใน Chrome
login-cookie-named-step-3 = ขยายคุกกี้ เลือก { $site } และค้นหาคุกกี้ชื่อ { $cookie }
login-cookie-named-step-4 = ดับเบิลคลิกที่ค่าแล้วคัดลอก
login-cookie-named-note = ค่า { $cookie } เพียงอย่างเดียวก็เพียงพอแล้ว ส่วนหัวของคุกกี้ทั้งหมดที่คัดลอกมาจากแท็บเครือข่ายก็ใช้ได้เช่นกัน
login-window-title = ลงชื่อเข้าใช้ { $provider }
login-use = ใช้ { $provider }
login-guest-title = โหมดผู้มาเยือน
login-guest-use = ใช้โหมดผู้มาเยือน
login-guest-detail = เรียกดูและเล่นโดยไม่ต้องมีบัญชี คลังเพลง รายการที่ชอบ และเพลย์ลิสต์ของคุณจะอยู่ห่างไกลจากการเข้าถึง
login-usage-consent = ช่วยเราประเมินจำนวนผู้ใช้ SonoraX
login-device-code = ใส่รหัสนี้ที่ { $url }
login-server-title = เชื่อมต่อกับเซิร์ฟเวอร์ Subsonic ของคุณ
login-server-detail = ป้อนที่อยู่ของเซิร์ฟเวอร์ Subsonic หรือ OpenSubsonic (Navidrome, Airsonic, Gonic, …) จากนั้นลงชื่อเข้าใช้ด้วยชื่อผู้ใช้และรหัสผ่านเซิร์ฟเวอร์ของคุณ เซสชันจะยังคงอยู่ในอุปกรณ์นี้
login-server-hint = https://music.example.com
login-username-hint = ชื่อผู้ใช้
login-password-hint = รหัสผ่าน
login-server-submit = เชื่อมต่อ
login-account-title = เลือกบัญชี
login-account-detail = เซสชันนี้ลงชื่อเข้าใช้บัญชี Google มากกว่าหนึ่งบัญชี เลือกอันที่Sonoraควรใช้
login-choose-title = ลงชื่อเข้าใช้ { $provider }
login-choose-detail = เลือกวิธีลงชื่อเข้าใช้ { $provider }
login-providers = บริการด้านดนตรี
# album and playlist pages
detail-playlist = เพลย์ลิสต์
# play button
play-pause = หยุดชั่วคราว
play-resume = ดำเนินการต่อ
play-shuffle = สับเปลี่ยน
# artist page
artist-eyebrow = ศิลปิน
artist-monthly-listeners =
    { $count ->
        [one] ผู้ฟังรายเดือน { $value }
       *[other] ผู้ฟังรายเดือน { $value }
    }
artist-popular = เป็นที่นิยม
artist-popular-eyebrow = สำรวจศิลปินคนนี้
artist-popular-empty = ยังไม่มีสิ่งใดให้เล่นจากศิลปินคนนี้
artist-popular-more = แสดงทั้งหมด
artist-popular-less = แสดงน้อยลง
artist-favorite-songs = เพลงโปรดของคุณ
artist-favorite-songs-count = { $count } เพลงในรายการโปรด
artist-releases = ข่าวประชาสัมพันธ์
artist-releases-more = แสดงทั้งหมด
artist-releases-less = แสดงน้อยลง
artist-filter-all = ทั้งหมด
artist-filter-albums = อัลบั้ม
artist-filter-singles = คนโสด
artist-filter-eps = อีพี
artist-appears-on = ปรากฏบน
album-also-like = คุณอาจจะชอบ
album-tab-albums = อัลบั้ม
album-tab-artists = ศิลปิน
album-label = ℗ { $label }
# user profile page
user-eyebrow = โปรไฟล์
user-followers =
    { $count ->
        [one] ผู้ติดตาม { $value }
       *[other] ผู้ติดตาม { $value }
    }
user-following =
    { $count ->
       *[other] { $value } กำลังติดตาม
    }
user-playlists = เพลย์ลิสต์สาธารณะ
user-playlists-empty = ยังไม่มีเพลย์ลิสต์สาธารณะ
# release kinds
release-album = อัลบั้ม
release-single = โสด
release-compilation = การรวบรวม
release-ep = EP
release-audiobook = หนังสือเสียง
release-podcast = พอดแคสต์
release-meta = { $year } • { $kind }
# home page
home-quick-picks = คัดสรรอย่างรวดเร็ว
home-quick-picks-empty = เช่นเดียวกับเพลงบางเพลงและเพลงเหล่านั้นจะปรากฏที่นี่
home-recently-added = เพิ่มล่าสุด
home-recently-played = เพิ่งเล่น
home-playlists = เพลย์ลิสต์
home-favorite-albums = อัลบั้มโปรด
home-artists = ศิลปิน
home-collection-albums = อัลบั้มจากคอลเลกชันของคุณ
# search page
search-placeholder = คุณอยากฟังอะไร?
search-browse = เรียกดูทั้งหมด
search-submit = ค้นหา
search-recent = การค้นหาล่าสุด
search-forget = ลบออกจากประวัติ
genre-empty = ยังไม่มีอะไรจะแสดงที่นี่
search-best-match = นัดที่ดีที่สุด
search-no-matches = ไม่พบรายการที่ตรงกัน
search-results = ผลลัพธ์
search-songs = เพลง
search-artists = ศิลปิน
search-albums-playlists = อัลบั้มและเพลย์ลิสต์
search-tag = { $kind } ·
search-saved =
    { $count ->
        [one] เพลง { $count } ในไลบรารี
       *[other] เพลง { $count } ในไลบรารี
    }
kind-song = เพลง
kind-artist = ศิลปิน
kind-album = อัลบั้ม
kind-playlist = เพลย์ลิสต์
# song page
song-eyebrow = เพลง
song-view-album = ดูอัลบั้ม
song-loading = กำลังโหลดข้อมูลเพลง...
song-about = เกี่ยวกับเพลงนี้
song-album = อัลบั้ม
song-released = เปิดตัวแล้ว
song-streams = สตรีม
song-position = ตำแหน่ง
song-label = ป้ายกำกับ
song-popularity = ความนิยม
song-popularity-value = { $value }%
song-disc-track = ดิสก์ { $disc }, แทร็ก { $track }
song-track = ติดตาม { $track }
song-credits = เครดิต
song-performed-by = ดำเนินการโดย
song-details = ประเภทและรายละเอียด
song-genres = ประเภท
song-language = ภาษา
song-content = เนื้อหา
song-explicit = มีเนื้อหาไม่เหมาะสำหรับเด็ก
song-clean = สะอาด
artist-about = เกี่ยวกับศิลปิน
artist-about-fallback = สำรวจเพลงยอดนิยมและการเผยแพร่ของศิลปิน
artist-about-open = ไปหาศิลปิน
song-copyright = © { $notice }
track-row-name = { $title } โดย { $artists }
# song languages
language-ar = ภาษาอาหรับ
language-de = เยอรมัน
language-en = อังกฤษ
language-es = สเปน
language-fr = ฝรั่งเศส
language-hi = ฮินดี
language-it = ภาษาอิตาลี
language-ja = ภาษาญี่ปุ่น
language-ko = เกาหลี
language-pt = โปรตุเกส
language-ru = ภาษารัสเซีย
language-tr = ภาษาตุรกี
language-uk = ภาษายูเครน
language-zh = จีน
language-zxx = ไม่มีเนื้อหาทางภาษา
# counts
count-songs =
    { $count ->
        [one] เพลง { $count }
       *[other] เพลง { $count }
    }
count-tracks =
    { $count ->
        [one] แทร็ก { $count }
       *[other] แทร็ก { $count }
    }
# running times
runtime-seconds = { $seconds } วินาที
runtime-minutes = { $minutes } นาที { $seconds } วินาที
runtime-hours = { $hours } ชม. { $minutes } นาที
# dates
date-just-now = เมื่อสักครู่
date-minute-ago = 1 นาทีที่แล้ว
date-minutes-ago = { $count } นาทีที่แล้ว
date-today = วันนี้ เวลา { $time }
date-yesterday = เมื่อวาน เวลา { $time }
date-time = { $date } เวลา { $time }
date-full = { $day } { $month } { $year }
month-1 = ม.ค.
month-2 = ก.พ.
month-3 = มี.ค.
month-4 = เม.ย.
month-5 = พ.ค.
month-6 = มิ.ย.
month-7 = ก.ค.
month-8 = ส.ค.
month-9 = ก.ย.
month-10 = ต.ค.
month-11 = พ.ย.
month-12 = ธ.ค.
# settings
settings-search = ค้นหาในการตั้งค่า
settings-search-results = ผลการค้นหา
# where a search result lives: a drill-in page under the page it opens from
settings-result-place = { $parent } › { $page }
settings-page-providers = ผู้ให้บริการ
settings-page-appearance = รูปลักษณ์
settings-page-player-bar = แถบเครื่องเล่น
settings-player-bar-layout = เค้าโครง
settings-player-bar-layout-detail = จัดเรียงข้อมูลแทร็ก ตัวควบคุมการเล่น และแถบความคืบหน้า
settings-player-bar-classic = คลาสสิค
settings-player-bar-compact = กะทัดรัด
settings-player-bar-centered = อยู่ตรงกลาง
settings-player-bar-surface = พื้นผิว
settings-player-bar-transparent = แถบเครื่องเล่นโปร่งใส
settings-player-bar-transparent-detail = ให้เนื้อหาและแถบด้านข้างเลื่อนผ่านใต้แถบเครื่องเล่นแบบโปร่งแสง
settings-player-bar-transparent-needs-blur = เปิดเอฟเฟกต์เบลอเพื่อให้เนื้อหาเลื่อนผ่านใต้แถบเครื่องเล่นแบบโปร่งแสง
settings-page-now-playing = กำลังเล่น
settings-page-lyrics = เนื้อเพลง
settings-page-playback = การเล่น
settings-page-sound = เสียง
settings-page-storage = พื้นที่จัดเก็บ
settings-page-discord = Discord
settings-page-scrobbling = สค็อบบลิง
settings-page-general = ทั่วไป
settings-page-about = เกี่ยวกับ
settings-tab-integrations = การเชื่อมต่อ
settings-about-name = SonoraX
settings-page-lyrics-sources = แหล่งเนื้อเพลง
settings-page-music-folders = โฟลเดอร์เพลง
settings-group-services = บริการสตรีม
settings-group-local-files = ไฟล์ในเครื่อง
settings-group-background = พื้นหลัง
settings-group-controls = การควบคุม
settings-group-display = การแสดงผล
settings-group-sources = แหล่งที่มา
settings-group-lyrics-order = ลำดับแหล่ง
settings-group-playing = กำลังเล่น
settings-group-cache = แทร็กที่เล่น
settings-group-downloads = ดาวน์โหลด
settings-group-protected = เสียงที่มี DRM
settings-group-output = เสียงออก
settings-group-status = สถานะ
settings-group-startup = การเริ่มต้น
settings-group-context-menus = เมนูบริบท
key-ctrl = Ctrl
key-super = Super
key-alt = Alt
key-shift = Shift
key-escape = Esc
key-enter = Enter
key-space = Space
key-tab = Tab
key-backspace = Backspace
key-delete = Delete
settings-lyrics-sources = แหล่งที่มาของเนื้อเพลง
settings-local-folder-count =
    { $count ->
        [one] โฟลเดอร์ { $count }
       *[other] โฟลเดอร์ { $count }
    }
settings-local-folder-none = ยังไม่มีโฟลเดอร์
settings-license-short = GPL-3.0 หรือใหม่กว่า
settings-theme = ธีม
settings-theme-detail = เลือกจานสีของแอปพลิเคชัน
settings-opacity = ความทึบ
settings-opacity-detail = ปรับความทึบของพื้นหลังแอป
settings-opacity-value = { $percent }%
settings-theme-folder = เปิดโฟลเดอร์
settings-theme-explore = สำรวจธีมของชุมชน
settings-theme-explore-detail = ค้นหาธีมที่คนอื่นสร้างและติดตั้งได้ในคลิกเดียว
settings-theme-delete = ลบธีม
settings-theme-delete-title = ลบ { $name }?
settings-theme-delete-detail = วิธีนี้จะลบออกจากโฟลเดอร์ธีมของคุณ มันไม่สามารถยกเลิกได้
settings-theme-unavailable = ธีมนี้สามารถใช้ได้เฉพาะเมื่อปิดใช้งานธีมแบบปรับได้เท่านั้น
settings-adaptive = ธีมที่ปรับเปลี่ยนได้
settings-adaptive-detail = ปรับสีธีมตามปกอัลบั้มที่กำลังเล่น
settings-ambient = พื้นหลังตามสีปก
settings-ambient-detail = ใช้สีจากปกอัลบั้มเป็นพื้นหลังของหน้ากำลังเล่น
settings-ambient-motion = พื้นหลังเคลื่อนไหว
settings-ambient-motion-detail = ให้สีพื้นหลังค่อย ๆ เคลื่อนไหว
settings-visualizer = วิชวลไลเซอร์
settings-visualizer-detail = วิธีการดึงสเปกตรัมเบื้องหลังงานศิลปะใน "กำลังเล่น"
settings-visualizer-style-none = ปิด
settings-visualizer-style-bars = บาร์
settings-visualizer-style-wave = คลื่น
settings-visualizer-style-both = บาร์และคลื่น
settings-visualizer-absolute = ละเว้นระดับเสียง
settings-visualizer-absolute-detail = วาดสเปกตรัมในระดับของแทร็ก ไม่ว่าSonoraจะดังแค่ไหนก็ตาม
settings-preview-title = ชื่อเพลง
settings-preview-artist = ศิลปิน
settings-preview-album = อัลบั้ม
settings-fullscreen-controls-autohide = ซ่อนการควบคุม
settings-fullscreen-controls-autohide-detail = ทำให้ตัวควบคุมการเล่นจางหายไปใน กำลังเล่น ขณะที่ตัวชี้วางอยู่
settings-os-fullscreen = ใช้ระบบปฏิบัติการแบบเต็มหน้าจอ
settings-os-fullscreen-detail = ทำให้การเล่นตอนนี้เต็มทั้งหน้าจอ เหมือนกับที่ F11 ทำ
settings-icons = ชุดไอคอน
settings-icons-detail = เลือกชุดไอคอนที่อินเทอร์เฟซดึงมา
settings-motion = ลดการเคลื่อนไหว
settings-motion-detail = ข้ามภาพเคลื่อนไหวและการเปลี่ยนอินเทอร์เฟซ
settings-pace = ความเร็วของแอนิเมชั่น
settings-pace-detail = ภาพเคลื่อนไหวของอินเทอร์เฟซเล่นเร็วแค่ไหน
settings-saver = ลดการใช้ทรัพยากรเมื่อไม่ได้ใช้งานหน้าต่าง
settings-saver-detail = การเปลี่ยนแปลงจะมีผลหลังเปิด SonoraX ใหม่
settings-corners = มุม
settings-corners-detail = พื้นผิวและส่วนควบคุมมีความโค้งมนเพียงใด
settings-blur = เอฟเฟกต์เบลอ
settings-blur-detail = เบลอเนื้อหาด้านหลังเมนูและปุ่มควบคุมแบบลอย
settings-blur-window = หน้าต่างเบลอ
settings-blur-window-detail = วาดหน้าต่างบนเดสก์ท็อปที่เบลอ ต้องการความทึบต่ำกว่า 100%
settings-font = ขนาดตัวอักษร
settings-font-detail = ขนาดข้อความพื้นฐาน ส่วนอื่นๆ จะปรับขนาดตามนั้น
settings-font-value = { $size } px
settings-font-smaller = ข้อความขนาดเล็กลง
settings-font-larger = ข้อความขนาดใหญ่ขึ้น
settings-startup = เริ่มที่
settings-startup-detail = หน้าจอSonoraเปิดขึ้นเมื่อเปิดตัว
settings-startup-no-guest = ไม่พร้อมใช้งานในโหมดผู้มาเยือน
settings-entries = รายการแถบด้านข้าง
settings-entries-detail = ส่วนที่ระบุไว้ในแถบด้านข้าง
settings-entries-pick = เลือกรายการ
settings-language = ภาษา
settings-language-detail = เลือกภาษาที่ใช้แสดงเมนูและข้อความของ SonoraX
settings-language-system = ตามภาษาของระบบ
settings-language-search = ค้นหาภาษา
settings-language-none = ไม่พบภาษา
settings-typeface = แบบอักษร
settings-typeface-detail = แบบอักษรที่ SonoraX ใช้ข้ามอินเทอร์เฟซ
settings-typeface-system = ค่าเริ่มต้น
settings-typeface-search = ค้นหาแบบอักษร
settings-typeface-none = ไม่พบแบบอักษร
settings-server-side-decorations = ของตกแต่งฝั่งเซิร์ฟเวอร์
settings-server-side-decorations-detail = ให้ผู้แต่งวาดแถบชื่อเรื่อง เส้นขอบ และเงา
settings-typeface-loading = กำลังโหลด...
settings-window-controls = การควบคุมหน้าต่าง
settings-window-controls-detail = วาดย่อเล็กสุด ขยายใหญ่สุด และปิดในแถบหัวเรื่อง
settings-traffic-light-controls = การควบคุมสัญญาณไฟจราจร
settings-traffic-light-controls-detail = วาดย่อเล็กสุด ขยายใหญ่สุด และปิดเป็นจุดสี
settings-window-rounding = มุมหน้าต่าง
settings-window-rounding-detail = มุมของหน้าต่างมีความโค้งมนแค่ไหน
settings-controls-side = ฝั่งควบคุม
settings-controls-side-detail = ส่วนควบคุมอยู่ที่ปลายแถบชื่อเรื่องด้านใด
settings-close-to-tray = ให้ทำงานในพื้นหลัง
settings-close-to-tray-detail = ให้Sonoraทำงานและเล่นต่อไปหลังจากที่หน้าต่างปิดลง
settings-tray-icon = แสดงในถาดระบบ
settings-tray-icon-detail = ใส่ไอคอนพร้อมตัวควบคุมการเล่นลงในถาดระบบ
settings-spotify-connect = แสดงเป็นอุปกรณ์ Spotify
settings-spotify-connect-name = ชื่ออุปกรณ์
settings-spotify-connect-name-detail = วิธีการตั้งชื่อ SonoraX ในรายการอุปกรณ์ของแอพของ Spotify
settings-spotify-connect-name-sonora = SonoraX
settings-spotify-connect-name-both = SonoraX (ชื่อคอมพิวเตอร์)
settings-spotify-connect-name-computer = ชื่อคอมพิวเตอร์
settings-spotify-connect-name-custom = กำหนดเอง
settings-spotify-connect-custom = ชื่อที่กำหนดเอง
settings-spotify-connect-custom-detail = ชื่อที่จะแสดง. หากเว้นว่างไว้ ระบบจะใช้Sonoraและชื่อของคอมพิวเตอร์
settings-spotify-connect-custom-hint = ชื่ออุปกรณ์
settings-spotify-connect-detail = แสดงรายการ SonoraX ในตัวเลือกอุปกรณ์ของแอพของ Spotify เพื่อให้คุณสามารถควบคุมได้จากโทรศัพท์ของคุณ และ Spotify จะแสดงสิ่งที่คุณเล่น
settings-discord = แสดงใน Discord
settings-discord-detail = ใส่เพลงที่คุณกำลังเล่นบนโปรไฟล์ Discord ของคุณ
settings-discord-name = ชื่อสถานะ
settings-discord-name-detail = สถานะที่เรียกว่าอะไรหลังจาก "กำลังฟัง" ที่เพื่อนของคุณเห็น
settings-discord-name-sonora = SonoraX
settings-discord-name-provider = ผู้ให้บริการ
settings-discord-name-music = ดนตรี
settings-discord-name-title = ชื่อเรื่อง
settings-discord-name-artist = ศิลปิน
settings-discord-name-artist-title = ศิลปิน - ชื่อเรื่อง
settings-discord-show-paused = แสดงเมื่อหยุดชั่วคราว
settings-discord-show-paused-detail = คงสถานะไว้ในโปรไฟล์ Discord ของคุณในขณะที่แทร็กถูกหยุดชั่วคราว
settings-discord-badge = แสดงตราผู้ให้บริการ
settings-discord-badge-detail = ทำเครื่องหมายสถานะด้วยไอคอนขนาดเล็กของบริการที่มาจากแทร็ก
settings-discord-anonymous = ซ่อนรายละเอียด
settings-discord-anonymous-detail = พูดเฉพาะเพลงที่กำลังเล่นอยู่ โดยไม่มีชื่อ ศิลปิน หรืองานศิลปะ
settings-discord-hide-on-connect = ซ่อนระหว่าง Spotify Connect
settings-discord-hide-on-connect-detail = ปล่อยให้สถานะเป็นของ Spotify ในขณะที่ SonoraX เล่นเป็นอุปกรณ์ Spotify หรืออุปกรณ์อื่นเล่น
settings-discord-buttons = ปุ่ม
settings-discord-buttons-detail = ลิงก์ภายใต้สถานะสำหรับให้เพื่อนของคุณเปิด ลิงก์หนึ่งไปยังเส้นทางการให้บริการ และอีกลิงก์ไปยังSonora
settings-discord-buttons-pick = เลือกปุ่ม
# what screen readers call the preview of the Discord status
settings-discord-preview = ดูตัวอย่างสถานะ Discord
settings-discord-preview-listening = กำลังฟัง { $name }
settings-discord-preview-music = ดนตรี
settings-discord-preview-elapsed = { $time } ผ่านไปแล้ว
settings-discord-preview-listen-on = ฟังได้ที่ { $provider }
# the Discord status when the track is left out of it
discord-listening = การฟังเพลง
get-sonora = รับSonora
settings-fullscreen-controls-autohide-automatic = อัตโนมัติ
settings-fullscreen-controls-autohide-always-hidden = ซ่อนอยู่เสมอ
settings-fullscreen-controls-autohide-always-shown = แสดงให้เห็นอยู่เสมอ
settings-normalisation = ปรับความดังให้เป็นปกติ
settings-normalisation-detail = รักษาเพลงด้วยระดับเสียงที่สม่ำเสมอ
settings-gapless = การเล่นที่ไม่มีช่องว่าง
settings-gapless-detail = เล่นเพลงหนึ่งไปยังเพลงถัดไปโดยไม่หยุด ซึ่งเป็นวิธีการเรียงลำดับอัลบั้ม
settings-crossfade = Crossfade
settings-crossfade-detail = ค่อย ๆ เบาท้ายเพลงพร้อมเร่งเสียงเพลงถัดไปเมื่อเสียงพร้อม ใช้ได้กับเพลงที่รองรับในตัวเล่นของ SonoraX
settings-crossfade-off = ปิด
settings-crossfade-seconds = { $seconds } วิ
settings-keep-tracks = เล่นเพลงต่อไป
settings-keep-tracks-detail = บันทึกสิ่งที่คุณเล่นโดยล็อกอยู่ในคอมพิวเตอร์เครื่องนี้ ดังนั้นการเล่นอีกครั้งจะไม่ดาวน์โหลดอะไรเลย การปิดฟีเจอร์นี้จะลบและปล่อยให้การดาวน์โหลดของคุณอยู่ตามลำพัง Spotify ติดตามการเปลี่ยนแปลงในครั้งถัดไปที่ SonoraX เริ่มทำงาน
settings-cache-size = ขนาดแคช
settings-cache-size-detail = แทร็กที่คุณเล่นอาจใช้พื้นที่เท่าใด อันที่เล่นนานที่สุดแล้วให้มีพื้นที่ก่อน และการดาวน์โหลดจะไม่นับรวม
settings-cache-size-value = { $count } GB
settings-cache-used = พื้นที่ที่ใช้
settings-cache-used-detail = แทร็กที่เก็บไว้หลังจากเล่นบนคอมพิวเตอร์เครื่องนี้ การล้างข้อมูลเหล่านั้นจะทำให้การดาวน์โหลดของคุณอยู่คนเดียว
settings-cache-clear = ล้างแคช
settings-downloads-used = พื้นที่ที่ใช้
settings-downloads-used-detail = เพลงที่คุณดาวน์โหลดใช้บนคอมพิวเตอร์เครื่องนี้
settings-offline-mode = โหมดออฟไลน์
settings-offline-mode-detail = แสดงและเล่นเฉพาะสิ่งที่คุณดาวน์โหลด
settings-downloads-remove = ลบทั้งหมด
settings-size-gb = { $size } GB
settings-size-mb = { $size } MB
settings-size-measuring = กำลังวัด...
settings-stream-quality = คุณภาพสตรีม
settings-stream-quality-detail = ระดับที่สูงขึ้นจะฟังดูดีขึ้นและใช้ข้อมูลมากขึ้น เมื่อแผนหรือแทร็กของคุณมีน้อยลง ระดับถัดไปจะเล่น
settings-stream-quality-hi-res = Hi-Res
settings-stream-quality-lossless = Lossless
settings-stream-quality-high = สูง
settings-stream-quality-normal = ปกติ
settings-stream-quality-low = ต่ำ
settings-stream-quality-flac = FLAC, { $depth } บิต { $rate } kHz
settings-stream-quality-flac-up-to = FLAC สูงสุด { $depth } บิต { $rate } kHz
settings-stream-quality-lossy = { $codec }, { $kbps } kbps
settings-stream-quality-lossy-up-to = { $codec } สูงสุด { $kbps } kbps
settings-stream-format = รูปแบบสตรีม
settings-stream-format-detail = สิ่งที่เซิร์ฟเวอร์แปลงแต่ละแทร็กเป็นก่อนที่จะส่ง
settings-stream-format-original = ต้นฉบับ
settings-stream-format-mp3 = MP3
settings-stream-format-opus = Opus
settings-stream-format-aac = AAC
settings-stream-bitrate = บิตเรตสูงสุด
settings-stream-bitrate-detail = เซิร์ฟเวอร์จะแปลงสิ่งใดก็ตามที่สูงกว่าลงมาเป็นอัตรานี้
settings-stream-bitrate-unlimited = ไม่จำกัด
settings-stream-bitrate-value = { $kbps } kbps
settings-stream-bitrate-hourly = ราว { $megabytes } MB/ชม.
settings-stream-bitrate-unlimited-detail = แต่ละไฟล์มีบิตเรตของตัวเอง
settings-output-device = อุปกรณ์เสียง
settings-output-device-detail = อุปกรณ์ที่ SonoraX ใช้เล่นเสียง
settings-output-device-system = ตามระบบ
settings-output-device-missing = อุปกรณ์เดิม (ไม่เชื่อมต่อ)
settings-output-device-direct = ต่ออุปกรณ์โดยตรง
settings-sample-rate = อัตราสุ่มเสียง
settings-sample-rate-detail = อัตราสุ่มเสียงของอุปกรณ์ SonoraX จะแปลงเสียงให้ตรงกับค่านี้
settings-sample-rate-value = { $khz } kHz
settings-sample-rate-lower = ลดอัตราสุ่ม
settings-sample-rate-higher = เพิ่มอัตราสุ่ม
settings-sample-rate-change = เปลี่ยน
settings-buffer = บัฟเฟอร์
settings-buffer-detail = จำนวนเสียงที่ SonoraX ส่งให้กับอุปกรณ์ในแต่ละครั้ง
settings-buffer-smaller = ลดบัฟเฟอร์
settings-buffer-larger = เพิ่มบัฟเฟอร์
settings-match-rate = ตามอัตราของเพลง
settings-match-rate-detail = เล่นแต่ละเพลงตามอัตราสุ่มเสียงของเพลงนั้น
settings-match-rate-direct-only = ต้องการอุปกรณ์ภายใต้ Direct to hardware
settings-output-idle = ไม่ได้เล่น
settings-output-device-loading = กำลังโหลดอุปกรณ์...
settings-output-failed = ไม่รองรับอุปกรณ์
settings-output-failed-detail = ข้อผิดพลาด: { $error }
settings-signal-source = ต้นทาง
settings-signal-normalized = ความดัง
settings-signal-gain = { $db } dB
settings-signal-resampled = แปลงเรต
settings-signal-khz = { $khz } kHz
settings-signal-kbps = { $kbps } kbps
settings-signal-bits = { $depth } บิต
settings-signal-source-detail = { $rate } · { $quality }
settings-signal-path = ทางเดินเสียง
settings-sleep = ตั้งเวลาปิดเครื่อง
settings-sleep-detail = ปล่อยให้เพลงหยุดเองหลังจากเวลาที่กำหนด เพื่อให้สามารถเล่นให้คุณนอนหลับได้
settings-stay-awake = ตื่นตัวในขณะที่เล่น
settings-stay-awake-detail = ป้องกันการนอนหลับขณะเล่นและเปิดหน้าจอไว้ในขณะที่กำลังเล่นอยู่เปิดอยู่
settings-sleep-configure = กำหนดค่า...
settings-sleep-off = ปิด
settings-sleep-end-of-track = สิ้นสุดการติดตาม
settings-sleep-minutes = { $count } นาที
settings-widevine = โมดูลไวด์ไวน์
settings-widevine-detail = เพลงของ Apple Music ได้รับการเข้ารหัสและต้องใช้โมดูล Widevine ของ Google SonoraXใช้อันที่ดาวน์โหลดจาก Google โดยได้รับความยินยอมจากคุณ หรือมิฉะนั้นก็สำเนาที่เบราว์เซอร์มีอยู่แล้ว
settings-widevine-none = เพลงของ Apple Music ได้รับการเข้ารหัสและต้องใช้โมดูล Widevine ของ Google ที่นี่ไม่มีเบราว์เซอร์ ดังนั้น SonoraX จึงสามารถดาวน์โหลดได้จาก Google โดยได้รับความยินยอมจากคุณ
settings-widevine-looking = กำลังดู...
settings-widevine-asking = รอคำตอบของคุณ
settings-widevine-fetching = กำลังดาวน์โหลด...
settings-widevine-installing = กำลังติดตั้ง...
settings-widevine-installed = พบได้ในเบราว์เซอร์
settings-widevine-fetched = ดาวน์โหลดจาก Google
settings-widevine-missing = ไม่ได้ติดตั้ง
settings-widevine-configured = กำหนดโดยสภาพแวดล้อม
settings-widevine-fetch = ดาวน์โหลด
settings-widevine-uninstall = ถอนการติดตั้ง
confirm-uninstall-widevine-title = ถอนการติดตั้งโมดูล Widevine หรือไม่
confirm-uninstall-widevine = SonoraXลบสำเนาที่ดาวน์โหลดมาจาก Google เพลง Apple Music จะหยุดเล่นจนกว่าจะดาวน์โหลดอีกครั้ง
confirm-remove-downloads-title = ลบการดาวน์โหลดทั้งหมดใช่ไหม
confirm-remove-downloads = ทุกเพลงที่คุณดาวน์โหลดจะถูกลบออกจากคอมพิวเตอร์เครื่องนี้ จากทุกบริการ พวกเขาจะสตรีมอีกครั้งจนกว่าคุณจะดาวน์โหลดอีกครั้ง
widevine-prompt-title = โมดูลไวด์ไวน์
widevine-prompt-wanted = เพลงของ Apple Music ได้รับการเข้ารหัสและต้องใช้โมดูล Widevine ของ Google ไม่มีเบราว์เซอร์ในคอมพิวเตอร์เครื่องนี้ SonoraX สามารถดาวน์โหลดได้จากเซิร์ฟเวอร์ของ Google ไฟล์เดียวกับที่ Chrome ติดตั้ง และเก็บไว้ในโฟลเดอร์ของตัวเอง
widevine-prompt-replace = เพลงของ Apple Music ได้รับการเข้ารหัสและต้องใช้โมดูล Widevine ของ Google เบราว์เซอร์บนคอมพิวเตอร์เครื่องนี้มีเบราว์เซอร์หนึ่งตัว และ SonoraX สามารถดาวน์โหลดสำเนาของ Google เพื่อใช้แทนได้ ซึ่งเป็นไฟล์เดียวกับที่ Chrome ติดตั้ง และเก็บไว้ในโฟลเดอร์ของมันเอง
widevine-prompt-downloading = กำลังดาวน์โหลด...
widevine-prompt-terms = ดาวน์โหลดเวอร์ชัน { $version } แล้ว การติดตั้งหมายถึงการยอมรับข้อกำหนดของ Google สำหรับโมดูล:
widevine-prompt-installing = กำลังติดตั้ง...
widevine-prompt-download = ดาวน์โหลด
widevine-prompt-later = ไม่ใช่ตอนนี้
widevine-prompt-accept = ยอมรับและติดตั้ง
widevine-prompt-decline = ปฏิเสธ
settings-equalizer = EQ
settings-equalizer-detail = กำหนดรูปทรงเสียงด้วยค่าที่ตั้งล่วงหน้าหรือเส้นโค้งของคุณเอง
settings-equalizer-preset-detail = เสียงสำเร็จรูปหรือหนึ่งในโปรไฟล์ของคุณเอง
settings-equalizer-custom = กำหนดเอง
settings-equalizer-flat = ราบ
settings-equalizer-bass-boost = เพิ่มเบส
settings-equalizer-bass-reducer = ลดเบส
settings-equalizer-treble-boost = เพิ่มแหลม
settings-equalizer-vocal = เสียงร้อง
settings-equalizer-rock = ร็อค
settings-equalizer-pop = ป๊อป
settings-equalizer-jazz = แจ๊ส
settings-equalizer-classical = คลาสสิค
settings-equalizer-electronic = อิเล็กทรอนิกส์
settings-equalizer-acoustic = อะคูสติก
settings-equalizer-loudness = ความดัง
equalizer-advanced = ขั้นสูง
equalizer-simple = เรียบง่าย
equalizer-sound = เสียง
equalizer-profile-default = ค่าเริ่มต้น
equalizer-profile-new = โปรไฟล์ { $number }
equalizer-profile-pasted = วางโปรไฟล์แล้ว
equalizer-profile-name-hint = ชื่อโปรไฟล์
equalizer-save-as = บันทึกเป็นโปรไฟล์
equalizer-rename = เปลี่ยนชื่อโปรไฟล์
equalizer-delete = ลบโปรไฟล์
equalizer-delete-confirm = ลบ “{ $name }” หรือไม่ สิ่งนี้ไม่สามารถยกเลิกได้
equalizer-undo = เลิกทำ
equalizer-redo = ทำซ้ำ
equalizer-bass = เบส
equalizer-treble = เสียงแหลม
equalizer-tone-reset = รีเซ็ตเป็น 0 dB
equalizer-preamp = ปรีแอมป์
equalizer-preamp-auto = อัตโนมัติ
equalizer-preamp-manual = คู่มือ
equalizer-decibels = dB
equalizer-decibels-value = { $db } dB
equalizer-hertz = Hz
equalizer-q = Q
equalizer-octaves = ต.ค
equalizer-no-gain = —
equalizer-filters = ตัวกรอง
equalizer-no-filters = ยังไม่มีตัวกรอง เพิ่มหนึ่งรายการ เลือกค่าที่ตั้งล่วงหน้าด้านบน หรือนำเข้าโปรไฟล์
equalizer-add-filter = เพิ่มตัวกรอง
equalizer-remove-filter = ลบตัวกรอง
equalizer-filter = กรอง { $number }
equalizer-filter-kind = ฟิลเตอร์ชนิด { $number }
equalizer-kind-peak = พีค
equalizer-kind-low-shelf = ชั้นวางต่ำ
equalizer-kind-high-shelf = ชั้นวางสูง
equalizer-kind-low-pass = ผ่านต่ำ
equalizer-kind-high-pass = ผ่านสูง
equalizer-kind-notch = รอยบาก
equalizer-kind-band-pass = วงดนตรีผ่าน
equalizer-kind-all-pass = ทุกอย่างผ่านไป
equalizer-transfer = นำเข้าและส่งออก
equalizer-import = นำเข้า
equalizer-export = ส่งออก
equalizer-import-file = นำเข้าไฟล์...
equalizer-paste = วาง
equalizer-copy = คัดลอก
equalizer-export-file = ส่งออกไฟล์...
equalizer-imported = นำเข้า { $name }
equalizer-imported-skipped =
    นำเข้า { $name } โดยไม่มี { $count ->
        [one] หนึ่งบรรทัด
       *[other] เส้น { $count }
    }
equalizer-import-nothing = ไม่พบการตั้งค่าอีควอไลเซอร์ที่จะนำเข้า
equalizer-import-too-many =
    ไม่ได้นำเข้า: { $count ->
        [one] ตัวกรองหนึ่งเปิดอยู่
       *[other] ตัวกรอง { $count } เปิดอยู่
    } มากกว่าที่เก็บโปรไฟล์
equalizer-import-too-large = ไม่ได้นำเข้า: ข้อความมีขนาดเกิน 1 MB ซึ่งใหญ่เกินไปสำหรับโปรไฟล์อีควอไลเซอร์
equalizer-filter-limit = โปรไฟล์รองรับตัวกรอง { $max } ได้สูงสุด
equalizer-paste-empty = คลิปบอร์ดไม่มีข้อความ
equalizer-copied = คัดลอกโปรไฟล์เป็นข้อความ Equalizer APO
equalizer-exported = ส่งออกโปรไฟล์แล้ว
equalizer-dialog-failed = ไม่สามารถเปิดกล่องโต้ตอบไฟล์ได้
equalizer-file-unreadable = ไม่สามารถอ่านไฟล์ได้
equalizer-file-unwritable = ไม่สามารถเขียนไฟล์ได้
equalizer-value-refused = เก็บค่าเดิมไว้
equalizer-skipped-line = สาย { $line }: { $reason }
equalizer-skipped-more =
    และ { $count ->
        [one] อีกหนึ่ง
       *[other] { $count } เพิ่มเติม
    }
equalizer-skip-unknown-type = ตัวกรองประเภทที่ไม่รู้จัก { $name }
equalizer-skip-unreadable = ไม่ใช่ตัวกรองที่Sonoraสามารถอ่านได้
equalizer-skip-no-frequency = ไม่มีความถี่
equalizer-skip-no-gain = ไม่มีกำไร
equalizer-skip-no-width = ไม่มี Q หรือแบนด์วิธ
equalizer-skip-expression = ค่าที่เขียนเป็นนิพจน์
equalizer-skip-no-preamp = ปรีแอมป์ไม่ใช่ตัวเลข
equalizer-skip-unsupported = ไม่รองรับ { $name }
equalizer-rules = กฎของอุปกรณ์
equalizer-rule-other = อุปกรณ์อื่นใด
equalizer-rule-playing = กำลังเล่นอยู่ครับ
equalizer-rule-disconnected = ไม่ได้เชื่อมต่อ
equalizer-rule-replaces = แทนที่กฎของมัน
equalizer-rule-choose-device = เลือกอุปกรณ์
equalizer-mode = โหมด
equalizer-preamp-level = ระดับปรีแอมป์
equalizer-no-gain-spoken = ไม่มีกำไร
equalizer-filter-frequency = กรองความถี่ { $number }
equalizer-filter-gain = กรองกำไรของ { $number }
equalizer-filter-width = กรองความกว้าง { $number }
equalizer-graph = การตอบสนองความถี่
equalizer-graph-detail =
    { $count ->
        [0] ไม่มีตัวกรอง
        [one] 1 ตัวกรอง
       *[other] ฟิลเตอร์ { $count }
    } ปรีแอมป์ที่ { $db } dB
equalizer-rule-device = อุปกรณ์
equalizer-rule-choose-sound = เลือกเสียง
equalizer-rule-add = เพิ่มกฎ
equalizer-rule-remove = ลบกฎ
equalizer-output-port = { $device } · { $port }
signal-path-codec-alac = ALAC
signal-path-codec-pcm = PCM
signal-path-codec-other = ตัวแปลงสัญญาณอื่น ๆ
equalizer-invalid-frequency = ความถี่ต้องอยู่ระหว่าง { $low } และ { $high } Hz
equalizer-invalid-gain = อัตราขยายต้องอยู่ระหว่าง { $low } และ { $high } dB
equalizer-invalid-q = Q ต้องอยู่ระหว่าง { $low } และ { $high }
equalizer-invalid-octaves = แบนด์วิดท์ต้องอยู่ระหว่างอ็อกเทฟ { $low } และ { $high }
equalizer-invalid-shelf-octaves = ชั้นวางใช้ Q ไม่ใช่แบนด์วิดท์
equalizer-invalid-preamp = ปรีแอมป์ต้องอยู่ระหว่าง { $low } และ { $high } dB
equalizer-overload = การตัดโดย { $db } dB
equalizer-overload-slight = การตัดเฉือนต่ำกว่า 0.1 dB
equalizer-graph-preamp = { $db } dB
equalizer-lower-preamp = ปรีแอมป์ตัวล่าง
equalizer-level = ระดับ
equalizer-status-bit-perfect = บิตที่สมบูรณ์แบบ
equalizer-status-bit-perfect-detail = สมบูรณ์แบบเล็กน้อยจากSonora
equalizer-status-server = เซิร์ฟเวอร์เสียงจะแปลงเป็น { $khz } kHz
equalizer-status-altered = ไม่สมบูรณ์แบบเล็กน้อยเนื่องจาก:
equalizer-status-separator = { ", " }
equalizer-status-more = +{ $count }
equalizer-status-equalized = EQ
equalizer-status-volume = ปริมาณ
equalizer-status-normalized = ทำให้เป็นมาตรฐาน
equalizer-status-rate = { $from } → { $to } kHz
equalizer-status-bits = { $from } → บิต { $to }
equalizer-status-float = ลอย → บิต { $to }
equalizer-status-channels =
    { $from ->
        [1] โมโน
        [2] สเตอริโอ
       *[other] { $from } ช
    } → { $to ->
        [1] โมโน
        [2] สเตอริโอ
       *[other] { $to } ช
    }
settings-lyrics = แสดงเนื้อเพลง
settings-lyrics-detail = ปิดเพื่อซ่อนเนื้อเพลงทุกหน้าและหยุดค้นหาเนื้อเพลง
settings-panel-lyrics-size = ขนาดข้อความในแผง
settings-panel-lyrics-size-detail = ขนาดของข้อความเนื้อเพลงในแผงด้านข้าง ที่ด้านบนของขนาดตัวอักษรพื้นฐาน
settings-fullscreen-lyrics-size = ขนาดข้อความในกำลังเล่น
settings-fullscreen-lyrics-size-detail = ขนาดของข้อความเนื้อเพลงในกำลังเล่นอยู่ ที่ด้านบนของขนาดตัวอักษรพื้นฐาน
settings-lyrics-size-value = { $size }%
settings-lyrics-for-local-files = ค้นหาเนื้อเพลงสำหรับไฟล์ในเครื่อง
settings-lyrics-for-local-files-detail = ใช้ข้อมูลเมตาจากไฟล์ในเครื่องเพื่อดึงเนื้อเพลงจากอินเทอร์เน็ต
settings-artwork-for-local-files = งานศิลปะสำหรับไฟล์ในเครื่อง
settings-artwork-for-local-files-detail = ใช้ข้อมูลเมตาจากไฟล์ในเครื่องเพื่อค้นหาเพลงคัฟเวอร์บน Deezer สำหรับสถานะ Discord ของคุณ
settings-karaoke-lyrics = คาราโอเกะ
settings-karaoke-lyrics-detail = เน้นเนื้อเพลงทีละคำเมื่อมีจังหวะเวลา
settings-blur-lyrics = เบลอเส้นที่ไม่ได้ใช้งาน
settings-blur-lyrics-detail = เบลอบรรทัดที่กำลังจะมาถึงและก่อนหน้าในแผงเนื้อเพลง
settings-lyrics-sample-first = ข้อมูลโดยพลการ
settings-lyrics-sample-second = แทรกข้อความตลกที่นี่
settings-lyrics-sample-japanese = ここに面白い文章を入れてください
settings-lyrics-sample-romanized = โคโค นิ โอโมชิโรอิ บุนโช หรือ อิเรเตะ คุดาไซ
settings-lyrics-sample-fourth = ฉันหวังว่าคนญี่ปุ่นนี้จะลองดู
settings-lyrics-sample-fifth = บรรทัดที่ห้า
settings-romanized-lyrics = เนื้อเพลงแบบโรมัน
settings-romanized-lyrics-detail = แสดงการออกเสียงที่สร้างขึ้นในท้องถิ่นสำหรับระบบการเขียนที่เลือก
settings-romanization-writing-systems = ระบบการเขียน
settings-lyrics-priority = ลำดับความสำคัญของผู้ให้บริการ
settings-lyrics-priority-detail = SonoraXแสดงเนื้อเพลงที่ซิงค์จากแหล่งสูงสุดที่มี
settings-lyrics-providers-detail = เลือกบริการที่จะค้นหาเนื้อเพลง
settings-lyrics-providers-selected =
    { $count ->
        [one] เลือก { $count } แล้ว
       *[other] เลือก { $count } แล้ว
    }
settings-lyrics-provider-local = ไฟล์ในเครื่อง
settings-lyrics-provider-subsonic = Subsonic
settings-lyrics-provider-spotify = Spotify
settings-lyrics-provider-youtube = YouTube Music
settings-lyrics-provider-apple-music = Apple Music
settings-lyrics-provider-musixmatch = Musixmatch
settings-lyrics-provider-lrclib = LRCLIB
settings-lyrics-provider-kugou = คูโกว
settings-lyrics-provider-netease = เน็ตอีส
settings-romanization-japanese = ภาษาญี่ปุ่น
settings-romanization-chinese = จีน
settings-romanization-korean = เกาหลี
settings-romanization-cyrillic = ซีริลลิก
settings-romanization-greek = ภาษากรีก
settings-romanization-arabic = ภาษาอาหรับ
settings-romanization-other = ระบบการเขียนอื่นๆ
settings-group-window = หน้าต่าง
settings-group-text = ข้อความ
settings-group-motion = การเคลื่อนไหว
settings-group-streaming = สตรีมมิ่ง
settings-group-spotify-connect = Spotify Connect
settings-group-sessions = เซสชัน
settings-sessions-button = แสดงปุ่มเซสชัน
settings-sessions-official = ใช้เซิร์ฟเวอร์ SonoraX Sessions อย่างเป็นทางการ
settings-sessions-server = ที่อยู่เซิร์ฟเวอร์
settings-sessions-server-hint = wss://sessions.example.com
settings-sessions-save = บันทึก
settings-adaptive-menu = เมนูบริบทที่ปรับเปลี่ยนได้
settings-adaptive-menu-detail = ละเว้นรายการที่แถวแสดงอยู่แล้ว เช่น อัลบั้มหรือศิลปิน
settings-accounts = บริการสตรีมมิ่ง
settings-accounts-detail = บริการที่อุปกรณ์นี้สามารถเล่นได้
settings-provider-current = ในการใช้งาน
settings-provider-sign-in = เข้าสู่ระบบ
settings-profile-account = { $provider } · { $account }
settings-sign-out = ออกจากระบบ
settings-lastfm = Last.fm
settings-lastfm-detail = SonoraX scrobbles ผ่านบัญชี Last.fm API ของคุณเอง สร้างหนึ่งรายการ จากนั้นวางคีย์และข้อมูลลับที่นี่
settings-librefm = Libre.fm
settings-listenbrainz = ListenBrainz
settings-listenbrainz-detail = วางโทเค็นผู้ใช้จากหน้าการตั้งค่า ListenBrainz ของคุณ
settings-maloja = มาโลจา
settings-maloja-detail = ชี้ SonoraX ไปที่เซิร์ฟเวอร์ Maloja ของคุณแล้ววางคีย์ API อันใดอันหนึ่ง
settings-scrobble-off = ไม่ได้เชื่อมต่อ
settings-scrobble-on = เชื่อมต่อแล้ว
settings-scrobble-waiting = กำลังรอเบราว์เซอร์ของคุณ...
settings-scrobble-as = Scrobbling เป็น { $name }
settings-scrobble-failed = ไม่สามารถเชื่อมต่อได้
settings-scrobble-connect = เชื่อมต่อ
settings-scrobble-disconnect = ตัดการเชื่อมต่อ
settings-scrobble-title = เชื่อมต่อ { $service }
settings-scrobble-request = รับคีย์ API
settings-scrobble-token-request = รับโทเค็น
settings-scrobble-key = คีย์เอพีไอ
settings-scrobble-secret = ความลับของ API
settings-scrobble-token = โทเค็นผู้ใช้
settings-scrobble-server = ที่อยู่เซิร์ฟเวอร์
settings-local-folder = โฟลเดอร์เพลง
settings-local-folder-empty = ไม่ได้กำหนดค่า
settings-choose-folder = เลือกโฟลเดอร์...
settings-add-folder = เพิ่มโฟลเดอร์
settings-remove-folder = ลบโฟลเดอร์
settings-rescan = สแกนอีกครั้ง
settings-scan-walking = กำลังสแกน...
settings-scan-progress = { $percent }%
settings-scan-done = เสร็จสิ้นใน { $seconds }s
settings-version = เวอร์ชัน
settings-version-detail = เวอร์ชัน SonoraX ที่กำลังใช้งาน
settings-license = ใบอนุญาต
settings-license-detail = GNU General Public License เวอร์ชัน 3 หรือใหม่กว่า
settings-source = ซอร์สโค้ด
settings-source-detail = แหล่งที่มาที่เกี่ยวข้องสำหรับรุ่นนี้
settings-team = ทีม
settings-role-lead-maintainer = หัวหน้าผู้ดูแล
settings-role-maintainer = ผู้ดูแล
settings-role-contributor = ผู้ร่วมให้ข้อมูล
settings-notice = ลิขสิทธิ์ © 2026 ผู้ร่วมให้ข้อมูลของSonoraX SonoraXไม่มีการรับประกันอย่างแน่นอน เป็นซอฟต์แวร์ฟรี และคุณสามารถแจกจ่ายต่อได้ภายใต้เงื่อนไขของ GNU General Public License เวอร์ชัน 3 หรือใหม่กว่า SonoraXไม่เป็นทางการและไม่มีส่วนเกี่ยวข้องกับ Spotify AB
# themes
theme-system = ระบบ
theme-dark = มืด
theme-light = เบา
theme-unavailable = { $name } (ไม่มีจำหน่าย)
theme-install-title = ติดตั้ง { $name }?
theme-install-detail = นี่จะเป็นการเพิ่มลงในโฟลเดอร์ธีมของคุณ
theme-install-confirm = ติดตั้ง
# corners
corners-square = สี่เหลี่ยมจัตุรัส
corners-subtle = บอบบาง
corners-rounded = โค้งมน
corners-round = รอบ
# motion
motion-system = ระบบ
motion-always = เสมอ
motion-never = ไม่เคย
pace-slow = ช้า
pace-base = มาตรฐาน
pace-quick = ด่วน
saver-off = ปิด
saver-fps = { $fps } FPS
toast-playlist-created = สร้างเพลย์ลิสต์แล้ว
toast-playlist-renamed = เปลี่ยนชื่อเพลย์ลิสต์แล้ว
toast-playlist-updated = อัปเดตเพลย์ลิสต์แล้ว
toast-playlist-cover-changed = หน้าปกเพลย์ลิสต์มีการเปลี่ยนแปลง
toast-playlist-cover-removed = นำหน้าปกเพลย์ลิสต์ออกแล้ว
toast-playlist-deleted = ลบเพลย์ลิสต์แล้ว
toast-local-delete-failed = ไฟล์แทร็กบางไฟล์ไม่สามารถลบได้
toast-playlist-added = เพิ่มเพลย์ลิสต์ลงในคลังเพลงของคุณแล้ว
toast-playlist-removed = เพลย์ลิสต์ถูกลบออกจากคลังเพลงของคุณ
toast-playlist-visibility = การเปิดเผยเพลย์ลิสต์เปลี่ยนไป
toast-track-added = เพิ่มไปยัง { $name } แล้ว
toast-track-removed = ลบออกจาก { $name }
toast-track-moved = ย้ายไปที่ { $name }
toast-track-removed-unnamed = ลบออกจากเพลย์ลิสต์แล้ว
toast-playlist-failed = ไม่สามารถบันทึกการเปลี่ยนแปลงนั้นได้
toast-playlist-busy = การเปลี่ยนแปลงอื่นยังคงดำเนินการอยู่
toast-playlist-signed-out = ลงชื่อเข้าใช้เพื่อเปลี่ยนเพลย์ลิสต์
toast-playlist-imported = นำเข้า { $name }
toast-playlist-imported-partly =
    นำเข้า { $name } แล้ว ยกเว้น { $count ->
        [one] หนึ่งเพลง
       *[other] เพลง { $count }
    } ที่ไม่ได้อยู่ในไลบรารีของคุณ
toast-playlist-import-empty = ไม่มีเพลงใน { $name } อยู่ในคลังเพลงของคุณ
toast-playlist-exported = ส่งออกเพลย์ลิสต์แล้ว
toast-file-dialog-failed = ไม่สามารถเปิดกล่องโต้ตอบไฟล์ได้
toast-queued-track = { $name } ถูกเพิ่มเข้าไปในคิว
toast-next-track = { $name } เล่นถัดไป
toast-last-track = { $name } เล่นครั้งสุดท้าย
toast-queued-album = เพิ่มอัลบั้มในคิวแล้ว
toast-next-album = อัลบั้มเล่นถัดไป
toast-last-album = การเล่นอัลบั้มครั้งสุดท้าย
toast-queued-playlist = เพิ่มเพลย์ลิสต์ลงในคิวแล้ว
toast-next-playlist = เพลย์ลิสต์เล่นถัดไป
toast-last-playlist = เพลย์ลิสต์จะเล่นครั้งสุดท้าย
toast-queued-artist = เพิ่มศิลปินเข้าคิวแล้ว
toast-next-artist = ศิลปินเล่นต่อไป
toast-last-artist = ศิลปินเล่นเพลงสุดท้าย
toast-queue-failed = ที่ไม่สามารถเพิ่มลงในคิวได้
toast-queue-local-elsewhere = ไฟล์ในเครื่องไม่สามารถจัดคิวบนอุปกรณ์อื่นได้
toast-queued-tracks = เพิ่มแทร็ก { $count } ลงในคิวแล้ว
toast-next-tracks = แทร็ก { $count } จะเล่นถัดไป
toast-last-tracks = แทร็ก { $count } เล่นครั้งสุดท้าย
toast-queue-cleared = ล้างคิวแล้ว
toast-queue-removed-track = ลบ { $name } ออกจากคิว
toast-queue-removed-tracks = ลบแทร็ก { $count } ออกจากคิว
toast-sleep-ended = ตัวตั้งเวลาปิดสิ้นสุดแล้ว
toast-radio-failed = ไม่สามารถเริ่มวิทยุสำหรับ { $name } ได้
toast-similar-failed = ไม่พบเพลงที่คล้ายกับ { $name }
toast-open-failed = ไม่สามารถเปิด { $name } ได้
toast-open-unsupported = { $name } ไม่ใช่ไฟล์เสียงที่ SonoraX สามารถเล่นได้
toast-open-all-failed = ไฟล์ { $count } ไม่สามารถเปิดได้
toast-open-some-failed = ไม่สามารถเปิดไฟล์บางไฟล์ได้
toast-keys-error = ข้อผิดพลาดของคีย์การเล่น
toast-keys-refused = Spotify อาจใช้งานไม่ได้ที่นี่
toast-why = ทำไม?
toast-sign-in-to-play = { $name } สตรีมไปยัง Listener ที่ลงชื่อเข้าใช้เท่านั้น
toast-restore-failed = { $name } ไม่สามารถลงชื่อเข้าใช้อีกครั้งได้ ดังนั้น SonoraX จึงดำเนินการต่อโดยไม่มีมัน
toast-session-invalidated = เซสชัน { $name } ของคุณไม่ถูกต้อง
toast-sign-in = เข้าสู่ระบบ
toast-track-unplayable = ไม่สามารถเล่น { $name } ได้
toast-playback-stopped = การเล่นหยุดลงหลังจากไม่สามารถเล่นหลายแทร็กติดต่อกันได้
toast-widevine-failed = การเล่นล้มเหลว
toast-widevine-borrowed = ลองอัพเดต Widevine
toast-open-settings = การตั้งค่า
toast-throttled = { $name } กำลังจำกัดการเล่นในขณะนี้ โปรดลองอีกครั้งในอีกสักครู่
toast-still-throttled = { $name } ยังคงจำกัดการเล่นอยู่ กดเล่นเพื่อลองอีกครั้ง
toast-library-add-failed = ไม่สามารถเพิ่ม { $name } ลงในไลบรารีของคุณได้
toast-library-remove-failed = ไม่สามารถลบ { $name } ออกจากไลบรารีของคุณได้
toast-library-added = เพิ่มลงในคลังเพลงของคุณแล้ว
toast-library-removed = ลบออกจากคลังเพลงของคุณแล้ว
toast-library-removed-one = ลบ { $name } ออกจากคลังเพลงของคุณ
toast-library-songs-removed =
    ลบ { $count ->
        [one] หนึ่งเพลง
       *[other] เพลง { $count }
    } ออกจากคลังเพลงของคุณ
toast-library-albums-removed =
    ลบ { $count ->
        [one] หนึ่งอัลบั้ม
       *[other] อัลบั้ม { $count }
    } ออกจากคลังเพลงของคุณ
toast-playlists-removed =
    ลบ { $count ->
        [one] เพลย์ลิสต์หนึ่งรายการ
       *[other] เพลย์ลิสต์ { $count }
    } ออกจากคลังเพลงของคุณ
toast-favorite-removed = ลบ { $name } ออกจากรายการโปรดของคุณ
toast-favorite-songs-removed =
    ลบ { $count ->
        [one] หนึ่งเพลง
       *[other] เพลง { $count }
    } ออกจากรายการโปรดของคุณ
toast-favorite-albums-removed =
    ลบ { $count ->
        [one] หนึ่งอัลบั้ม
       *[other] อัลบั้ม { $count }
    } ออกจากรายการโปรดของคุณ
toast-favorite-artists-removed =
    ลบ { $count ->
        [one] ศิลปินคนหนึ่ง
       *[other] ศิลปิน { $count }
    } ออกจากรายการโปรดของคุณ
toast-tracks-removed =
    ลบ { $count ->
        [one] หนึ่งเพลง
       *[other] เพลง { $count }
    } ออกจาก { $name }
toast-tracks-removed-unnamed =
    ลบ { $count ->
        [one] หนึ่งเพลง
       *[other] เพลง { $count }
    } ออกจากเพลย์ลิสต์
toast-unpinned = เลิกปักหมุด { $name } แล้ว
# lyrics
lyrics-title = เนื้อเพลง
lyrics-untimed = เนื้อเพลงไม่ซิงก์
lyrics-auto-working = กำลังซิงก์เนื้อเพลง…
lyrics-auto-credit = ซิงก์เนื้อเพลงบนเครื่อง
lyrics-file-add = เพิ่ม .lrc
lyrics-file-replace = เปลี่ยน .lrc
lyrics-file-reset = ใช้เนื้อเดิม
lyrics-file-saved = บันทึกไฟล์เนื้อเพลงแล้ว
lyrics-file-removed = กลับไปใช้เนื้อเพลงเดิมแล้ว
lyrics-file-import-failed = บันทึกเนื้อเพลงไม่ได้ เลือกไฟล์ .lrc แบบ UTF-8 ที่มีเวลาซิงก์และขนาดไม่เกิน 1 MB
lyrics-file-remove-failed = เอาไฟล์เนื้อเพลงออกไม่ได้
lyrics-idle = เล่นบางอย่างเพื่อดูเนื้อเพลง
lyrics-loading = กำลังมองหาเนื้อเพลง...
lyrics-missing = ไม่พบเนื้อเพลง ขออภัย!
lyrics-instrumental = เพลงนี้เป็นเครื่องมือ
lyrics-failed = ไม่สามารถเข้าถึงบริการเนื้อเพลงได้
lyrics-follow = ติดตามเพลงอีกครั้ง.
lyrics-source = เนื้อเพลงจาก { $source }
lyrics-source-local = เนื้อเพลงจากไฟล์ในเครื่อง
lyrics-writers = เขียนโดย { $writers }
lyrics-instrumental-break = เบรกเครื่องมือ
update-available = SonoraX { $version } พร้อมให้อัปเดตแล้ว
update-detail = คุณอยู่ที่ { $running } กำลังติดตั้งอัปเดตใหม่
update-detail-notes = คุณอยู่ที่ { $running } อ่านรายละเอียดอัปเดต SonoraX
update-notes = มีอะไรใหม่
update-now = อัปเดต
update-working = กำลังดาวน์โหลดอัปเดต...
update-failed = ไม่สามารถติดตั้งการอัปเดตได้ ลองอีกครั้งจากหน้าการเผยแพร่
update-never = อย่าตรวจสอบการอัปเดต
notice-never = อย่าแสดงอีก
requirement-audio-bridge-title = SonoraXไม่สามารถเข้าถึงเซิร์ฟเวอร์เสียงของคุณ
requirement-audio-bridge =
    { $server ->
        [pulseaudio] PulseAudio กำลังทำงานอยู่ แต่ ALSA ไม่มีเส้นทางไป จึงไม่ได้ยินสิ่งที่คุณเล่น ติดตั้ง pulseaudio-alsa และรีสตาร์ท SonoraX
       *[pipewire] PipeWire กำลังทำงานอยู่ แต่ ALSA ไม่มีเส้นทางไป ดังนั้นจึงไม่มีใครได้ยินสิ่งที่คุณเล่น ติดตั้ง Pipewire-alsa และรีสตาร์ท SonoraX
    }
requirement-audio-output-title = ไม่มีเสียงออก
requirement-audio-output = SonoraXไม่สามารถเปิดอุปกรณ์เสียงของคุณได้ จึงไม่ได้ยินสิ่งใดที่คุณเล่น ตรวจสอบว่าอุปกรณ์เชื่อมต่ออยู่และไม่ได้ถือโดยแอปอื่น
requirement-web-engine-title = การลงชื่อเข้าใช้เบราว์เซอร์ไม่พร้อมใช้งาน
requirement-web-engine = ติดตั้ง webkit2gtk-4.1 เพื่อลงชื่อเข้าใช้ YouTube Music, Deezer หรือ Apple Music ในหน้าต่างเบราว์เซอร์ ในระหว่างนี้ ให้วางคุกกี้ของคุณแทน
requirement-help = สิ่งที่ต้องติดตั้ง
settings-check-updates = อัปเดตอัตโนมัติ
settings-check-updates-detail = SonoraX จะติดตั้งเวอร์ชันใหม่โดยอัตโนมัติเมื่อเปิดโปรแกรม หากมีอัปเดตระหว่างที่กำลังใช้งาน จะแจ้งให้ทราบ
settings-auto-updates-enabled = เปิด
settings-update-channel = ช่องทางการเผยแพร่
settings-update-channel-detail = เสถียรติดตามการเปิดตัว Nightly ดำเนินรอยตามโครงสร้างหลักที่สร้างขึ้นทุกคืน ซึ่งจะได้รับการเปลี่ยนแปลงก่อนและมีการทดสอบน้อยกว่า
update-channel-stable = มีเสถียรภาพ
update-channel-nightly = ทุกคืน
settings-log = ไฟล์บันทึก
settings-log-detail = สิ่งที่Sonoraเขียนขณะวิ่ง แนบไปกับรายงานข้อผิดพลาด
settings-log-open = เปิดบันทึก
# tags
tags-edit-title = แก้ไขแท็ก
tags-sheet-song = เพลง
tags-sheet-album = อัลบั้ม
tags-sheet-details = รายละเอียด
tags-title = ชื่อเรื่อง
tags-artist = ศิลปิน
tags-artists = ศิลปิน
tags-add-artist = เพิ่มศิลปิน
tags-remove-artist = ลบศิลปินคนนี้
tags-track = หมายเลขแทร็ก
tags-track-total = แทร็กที่วางจำหน่าย
tags-disc = หมายเลขดิสก์
tags-disc-total = แผ่นดิสก์ที่วางจำหน่าย
tags-album = อัลบั้ม
tags-album-artist = ศิลปินอัลบั้ม
tags-year = ปี
tags-genre = ประเภท
tags-composer = ผู้แต่ง
tags-publisher = สำนักพิมพ์
tags-isrc = ISRC
tags-comment = ความคิดเห็น
toast-tags-saved = บันทึกแท็กสำหรับ { $name }
toast-tags-failed = ไม่สามารถบันทึกแท็กได้
toast-output-select-failed = ไม่สามารถเลือก { $name } ได้
toast-output-default-failed = ไม่สามารถเลือกอุปกรณ์เริ่มต้นของระบบได้
toast-output-direct-failed = ไม่สามารถเล่นบนอุปกรณ์ได้โดยตรง
toast-output-rate-failed = เปลี่ยนอัตราสุ่มเสียงไม่ได้
nav-pin = พิน
toast-library-pin-failed = ไม่สามารถอัปเดตพินได้
toast-jam-unreadable = ลิงก์นั้นไม่ได้นำไปสู่ ​​Jam ที่คุณสามารถเข้าร่วมได้
toast-jam-needs-connect = เปิด "แสดงเป็นอุปกรณ์ Spotify" ในการตั้งค่าเพื่อเข้าร่วม Jam
toast-jam-failed = ไม่สามารถเริ่มหรือเข้าร่วม Jam ได้
toast-jam-ended = แจมจบแล้ว
toast-jam-left = คุณออกจาก Jam แล้ว
toast-jam-leave-failed = ไม่สามารถสิ้นสุดหรือออกจาก Jam ได้
toast-session-needs-provider = ลงชื่อเข้าใช้บริการสตรีมมิ่งเพื่อเริ่มเซสชัน
toast-session-not-found = ไม่มีเซสชันใดใช้รหัสนั้น
toast-session-provider = เซสชันนี้เล่นจาก { $name } ลงชื่อเข้าใช้ { $name } เพื่อเข้าร่วม
toast-session-full = เซสชั่นนั้นเต็มแล้ว
toast-session-rate-limited = รหัสผิดเยอะมาก โปรดลองอีกครั้งในอีกสักครู่
toast-session-too-many = เครือข่ายของคุณโฮสต์เซสชันมากที่สุดเท่าที่จะเป็นไปได้แล้ว จบอันหนึ่งเพื่อเริ่มอันอื่น
toast-session-unreachable = ไม่สามารถเข้าถึงเซสชัน SonoraX ลองอีกครั้งในภายหลัง
toast-session-failed = ไม่สามารถเริ่มหรือเข้าร่วมเซสชันได้
toast-session-lost = สูญเสียการเชื่อมต่อกับเซสชัน
toast-session-ended = เซสชันสิ้นสุดแล้ว
toast-session-left = คุณออกจากเซสชันแล้ว
toast-session-kicked = โฮสต์ลบคุณออกจากเซสชัน
toast-session-code-copied = คัดลอกรหัสแล้ว
toast-sessions-server-saved = บันทึกเซิร์ฟเวอร์เซสชันแล้ว
toast-sessions-server-empty = พิมพ์ที่อยู่เซิร์ฟเวอร์ของคุณก่อน
toast-sessions-server-malformed = ที่ไม่ได้อ่านเป็นที่อยู่เซิร์ฟเวอร์
toast-sessions-server-silent = ไม่มีเซิร์ฟเวอร์ SonoraX Sessions ตอบกลับตามที่อยู่นี้ ดังนั้นจึงไม่ได้รับการบันทึก
nav-nothing-pinned = ยังไม่ปักหมุด
nav-pins-alphabetical = ตามตัวอักษร
nav-pins-kind = ตามประเภท
nav-show-full-library = ดูคลังทั้งหมด
nav-return-top = กลับด้านบน
# trouble
trouble-offline = ไม่มีการเชื่อมต่อ
trouble-offline-detail = ตรวจสอบการเชื่อมต่ออินเทอร์เน็ตของคุณแล้วลองอีกครั้ง
trouble-not-loaded = ไม่สามารถโหลดได้
trouble-retry = ลองอีกครั้ง
toast-offline = ไม่มีการเชื่อมต่อ ไม่มีอะไรจะสตรีมจนกว่าจะกลับมา
toast-offline-unsupported = { $name } เล่นการดาวน์โหลดเมื่อออนไลน์เท่านั้น
toast-offline-mode = คุณออฟไลน์อยู่ ออนไลน์เพื่อสตรีมเพลงนี้
offline-title = คุณออฟไลน์อยู่
offline-go-online = ออนไลน์
toast-download-failed = ไม่สามารถแสดงรายการแทร็กที่จะดาวน์โหลดได้
toast-percent = { $percent }%
toast-download-started = เริ่มดาวน์โหลด { $name }
toast-downloads-started =
    { $count ->
        [one] เริ่มดาวน์โหลดเพลง { $count }
       *[other] เริ่มดาวน์โหลดเพลง { $count }
    }
toast-download-cancelled = ยกเลิกการดาวน์โหลด { $name }
toast-downloads-cancelled =
    { $count ->
        [one] ยกเลิกการดาวน์โหลดแทร็ก { $count }
       *[other] ยกเลิกการดาวน์โหลดแทร็ก { $count }
    }
toast-settings-broken = แก้ไขบรรทัด { $name } ของ settings.json เพื่อบันทึกการเปลี่ยนแปลง
toast-tray-unavailable = ไม่มีถาดระบบให้ใส่ไอคอน
toast-undo = เลิกทำ
toast-link-copied = คัดลอกลิงก์แล้ว
toast-link-sign-in = ลงชื่อเข้าใช้ { $name } เพื่อเปิดลิงก์นี้
toast-link-unreadable = ลิงก์นั้นไม่ได้นำไปสู่ที่ที่Sonoraสามารถเปิดได้
toast-theme-link-unreadable = ลิงก์ธีมนั้นเสียหายหรือมีอย่างอื่นที่ไม่ใช่ธีม
toast-theme-install-failed = ไม่สามารถติดตั้ง { $name }
toast-theme-delete-failed = ไม่สามารถลบ { $name } ได้
toast-code-copied = คัดลอกรหัสแล้ว
menu-copy-link-none = นี่ไม่มีลิงก์ให้แชร์
menu-copy-link-local = ศิลปินและเพลย์ลิสต์ท้องถิ่นไม่มีลิงก์
toast-settings-save-failed = ไม่สามารถบันทึกการตั้งค่าได้
toast-settings-overwritten = settings.json มีการเปลี่ยนแปลงบนดิสก์ ดังนั้นการเปลี่ยนแปลงครั้งล่าสุดของคุณจึงไม่ถูกบันทึก
toast-widevine-download-failed = ไม่สามารถดาวน์โหลด Widevine ได้
toast-widevine-install-failed = ไม่สามารถติดตั้ง Widevine ได้
toast-widevine-remove-failed = Widevine ไม่สามารถลบออกได้
toast-library-scanned =
    { $count ->
        [one] สแกนคลังเพลงแล้ว พบหนึ่งเพลง
       *[other] สแกนไลบรารีแล้ว พบเพลง { $count }
    }
toast-scan-failed = ไม่สามารถสแกนโฟลเดอร์เพลงของคุณได้
toast-folder-removed = ลบ { $name } แล้ว
toast-folder-missing = { $name } ไม่ได้อยู่ในดิสก์อีกต่อไป
toast-history-removed =
    { $count ->
        [one] ลบออกจากประวัติศาสตร์แล้ว
       *[other] ลบเพลง { $count } ออกจากประวัติ
    }
toast-history-remove-failed = ไม่สามารถลบเพลงออกจากประวัติศาสตร์ได้
toast-history-restore-failed = ไม่สามารถกู้คืนประวัติได้
toast-history-clear-failed = ไม่สามารถล้างประวัติได้
# power
wake-reason = กำลังเล่นดนตรี
toast-sample-plain = สร้างกิจกรรมแล้ว
toast-sample-done = บันทึกลงในคลังเพลงของคุณแล้ว
toast-sample-info = มาถึงก่อนงานเริ่ม 10 นาที
toast-sample-warning = เซสชั่นของคุณจะสิ้นสุดในห้านาที
toast-sample-failed = ไม่สามารถเข้าถึงเซิร์ฟเวอร์ได้
toast-sample-loading = กำลังอัปโหลดเพลงของคุณ
toast-sample-event = สร้างกิจกรรมแล้ว
toast-sample-event-detail = วันอาทิตย์ที่ 3 ธันวาคม เวลา 09.00 น
toast-sample-delete = ลบ { $name } แล้ว
toast-sample-undo = เลิกทำ
toast-sample-cancel = ยกเลิก
toast-sample-undone = เลิกทำ
toast-sample-queued = { $name } ถูกเพิ่มเข้าไปในคิว
toast-sample-long = ข้อความนี้พูดได้ดีกว่าข้อความอื่นๆ ดังนั้นข้อความจึงต้องตัดบรรทัดที่สองหรืออาจเป็นบรรทัดที่สามโดยไม่ต้องกดปุ่มปิดออกจากการ์ด
toast-sample-saving = กำลังบันทึก { $name }
toast-sample-saved = บันทึก { $name } แล้ว
toast-sample-save-failed = ไม่สามารถบันทึก { $name } ได้
toast-sample-save-failed-detail = เซิร์ฟเวอร์ตอบ 503 บริการไม่พร้อมใช้งาน
toast-sample-lasting = อันนี้จะอยู่จนกว่าคุณจะปิด
toast-sample-burst = ขนมปังปิ้งเบอร์ { $name }
# Frequency response graph
response-graph-hz = { $value } Hz
response-graph-khz = { $value } kHz
response-graph-db = { $value } dB
response-graph-readout =
    { $frequency } · { $direction ->
        [up] +{ $gain }
        [down] -{ $gain }
       *[flat] { $gain }
    } dB
